use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    // Data structures for quiz evaluation
    #[derive(Copy, Clone)]
    pub struct QuizResult {
        pub score: u32,
        pub total_points: u32,
        pub percentage: u32,
        pub passed: bool,
        pub feedback: u32, // Encoded feedback message
    }

    #[derive(Copy, Clone)]
    pub struct InstructorAnalytics {
        pub class_average: u32,
        pub difficulty_rating: u32,
        pub question_analysis: u32,
        pub improvement_suggestions: u32,
    }

    #[derive(Copy, Clone)]
    pub struct StudentFeedback {
        pub personal_score: u32,
        pub correct_answers: u32,
        pub study_recommendations: u32,
        pub next_quiz_suggestions: u32,
    }

    // ✅ CONSOLIDATED: All MXE quiz data in ONE array to reduce argument count
    // This reduces args from 26 to 20 and prevents "out of memory" error
    #[instruction]
    pub fn quiz_evaluation(
        user_answers: Enc<Shared, [u32; 2]>,  // Student answers encrypted with same key/nonce [answer1, answer2]
        quiz_data: Enc<Mxe, [u32; 8]>,        // ALL quiz data in one array: [correct1, correct2, points1, points2, threshold, stat1, stat2, stat3]
        student: Shared,                      // Re-encrypt result for student
        instructor: Shared,                   // Re-encrypt analytics for instructor
        quiz_creator: Shared                  // Re-encrypt full results for quiz creator
    ) -> (Enc<Shared, QuizResult>, Enc<Shared, InstructorAnalytics>, Enc<Shared, StudentFeedback>) {
        // Decrypt inputs
        let user_ans = user_answers.to_arcis();
        let ua1 = user_ans[0];
        let ua2 = user_ans[1];
        
        // Decrypt consolidated quiz data and extract individual values
        let qd = quiz_data.to_arcis();
        let ca1 = qd[0];  // correct_answer1
        let ca2 = qd[1];  // correct_answer2
        let p1 = qd[2];   // points1
        let p2 = qd[3];   // points2
        let threshold = qd[4];  // passing_threshold
        let total_students = qd[5];     // class_stat1
        let total_class_score = qd[6];  // class_stat2
        let completion_count = qd[7];   // class_stat3
        
        // Calculate score
        let total_points = p1 + p2;
        let diff1 = ua1 - ca1;
        let diff2 = ua2 - ca2;
        
        // Use constant-time comparison
        let is_correct1 = (diff1 == 0u32) as u32;
        let is_correct2 = (diff2 == 0u32) as u32;
        
        let points_earned1 = is_correct1 * p1;
        let points_earned2 = is_correct2 * p2;
        let earned_points = points_earned1 + points_earned2;
        
        // Calculate percentage using constant-time operations to prevent side-channel leaks
        let is_zero = (total_points == 0u32) as u32;
        let safe_divisor = total_points + is_zero; // Ensures divisor is never 0
        let percentage = (earned_points * 100u32) / safe_divisor;
        let final_percentage = percentage * (1u32 - is_zero); // Zero out result if total_points was 0
        
        let passed = final_percentage >= threshold;
        
        // Quiz result for student
        let quiz_result = QuizResult {
            score: earned_points,
            total_points,
            percentage: final_percentage,
            passed,
            feedback: (passed as u32), // 1 = "Great job!", 0 = "Keep studying!"
        };
        
        // Calculate proper class analytics with aggregation using constant-time operations
        let completion_is_zero = (completion_count == 0u32) as u32;
        let students_is_zero = (total_students == 0u32) as u32;
        
        let safe_completion_count = completion_count + completion_is_zero;
        let safe_student_count = total_students + students_is_zero;
        
        let class_average_raw = (total_class_score * 100u32) / (safe_completion_count * total_points);
        let class_average = class_average_raw * (1u32 - completion_is_zero);
        
        let class_completion_rate_raw = (completion_count * 100u32) / safe_student_count;
        let _class_completion_rate = class_completion_rate_raw * (1u32 - students_is_zero);
        
        // Analytics for instructor
        let instructor_analytics = InstructorAnalytics {
            class_average, // Properly aggregated across all students
            difficulty_rating: (p1 + p2) / 2u32,
            question_analysis: is_correct1 + is_correct2,
            improvement_suggestions: (class_average < 70u32) as u32,
        };
        
        // Personal feedback for student
        let student_feedback = StudentFeedback {
            personal_score: final_percentage,
            correct_answers: is_correct1 + is_correct2,
            study_recommendations: (final_percentage < 70u32) as u32,
            next_quiz_suggestions: (passed as u32),
        };
        
        // Re-encrypt results for different stakeholders
        let student_result = student.from_arcis(quiz_result);
        let instructor_result = instructor.from_arcis(instructor_analytics);
        let creator_result = quiz_creator.from_arcis(student_feedback);
        
        (student_result, instructor_result, creator_result)
    }
}
