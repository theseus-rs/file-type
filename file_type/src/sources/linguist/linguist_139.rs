use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_139: FileFormat = FileFormat {
    id: 139,
    source_type: SourceType::Linguist,
    name: "GraphQL",
    extensions: &["gql", "graphql", "graphqls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
