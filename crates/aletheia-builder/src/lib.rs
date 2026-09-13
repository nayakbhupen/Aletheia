pub mod pipeline;

pub use pipeline::DatasetBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_synthesis() {
        let builder = DatasetBuilder::new(10);
        let (items, report) = builder.build_dataset().unwrap();
        assert_eq!(items.len(), 10);
        assert!(report.mean_isomorphism_score > 0.7);
    }
}
