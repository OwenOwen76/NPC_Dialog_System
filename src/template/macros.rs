#[macro_export]
macro_rules! idea {
    ($mem_key:literal => $cond_input:expr, $(($resp:expr, $weight:expr)),* $(,)?) => {
        (
            $mem_key.to_string(),
            idea!(@cond $cond_input),
            $crate::utility::math::WeightedPool {
                variants: vec![ $( ($resp, $weight) ),* ],
            }
        )
    };

    ($cond_input:expr, $(($resp:expr, $weight:expr)),* $(,)?) => {
        (
            idea!(@cond $cond_input),
            $crate::utility::math::WeightedPool {
                variants: vec![ $( ($resp, $weight) ),* ],
            }
        )
    };

    (@cond $word:literal) => { $crate::utility::input::KeyCondition::Word($word) };
    (@cond $logic:expr) => { $logic };
}
