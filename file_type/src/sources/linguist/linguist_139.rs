use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_139: FileType = FileType {
    file_format: &FileFormat {
        id: 139,
        source_type: SourceType::Linguist,
        name: "GraphQL",
        extensions: &["gql", "graphql", "graphqls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
