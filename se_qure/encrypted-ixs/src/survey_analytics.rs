use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    // Data structures for survey analytics
    #[derive(Copy, Clone)]
    pub struct SurveyAnalytics {
        pub total_responses: u32,
        pub average_rating: u32,
        pub completion_rate: u32,
        pub response_pattern: u32,
    }

    #[derive(Copy, Clone)]
    pub struct PublicSurveySummary {
        pub total_responses: u32,
        pub completion_rate: u32,
    }

    #[derive(Copy, Clone)]
    pub struct RespondentFeedback {
        pub response_id: u32,
        pub feedback_score: u32,
        pub completion_time: u32,
    }

    // ✅ UPDATED: Survey analytics with array type (Following Nico's advice for shared encryption)
    // Since all 6 values share the same encryption key/nonce, we use Enc<Shared, [u32; 6]>
    #[instruction]
    pub fn survey_analytics(
        user_data: Enc<Shared, [u32; 6]>,  // All 6 values encrypted with same key/nonce
                                            // [answer1, answer2, question_type1, question_type2, total_responses, completion_rate]
        survey_creator: Shared,             // Re-encrypt full analytics for survey creator
        public_viewer: Shared,              // Re-encrypt limited summary for public viewers
        respondent: Shared                  // Re-encrypt feedback for the respondent
    ) -> (Enc<Shared, SurveyAnalytics>, Enc<Shared, PublicSurveySummary>, Enc<Shared, RespondentFeedback>) {
        // Decrypt and extract values from the array
        let data = user_data.to_arcis();
        let a1 = data[0];   // answer1
        let a2 = data[1];   // answer2
        let qt1 = data[2];  // question_type1
        let qt2 = data[3];  // question_type2
        let total_resp = data[4];  // total_responses
        let completion = data[5];  // completion_rate
        
        // Calculate comprehensive analytics
        let average_rating = (a1 + a2) / 2u32;
        let response_pattern = a1 + a2 + qt1 + qt2;
        
        // Full analytics for survey creator
        let full_analytics = SurveyAnalytics {
            total_responses: total_resp,
            average_rating,
            completion_rate: completion,
            response_pattern,
        };
        
        // Limited public summary
        let public_summary = PublicSurveySummary {
            total_responses: total_resp,
            completion_rate: completion,
        };
        
        // Personal feedback for respondent
        let respondent_feedback = RespondentFeedback {
            response_id: a1 + a2, // Unique response identifier
            feedback_score: average_rating,
            completion_time: qt1 + qt2, // Time-based metric
        };
        
        // Re-encrypt results for different stakeholders
        let creator_analytics = survey_creator.from_arcis(full_analytics);
        let public_analytics = public_viewer.from_arcis(public_summary);
        let respondent_result = respondent.from_arcis(respondent_feedback);
        
        (creator_analytics, public_analytics, respondent_result)
    }
}
