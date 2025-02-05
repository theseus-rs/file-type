use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_198: FileFormat = FileFormat {
    id: 198,
    source_type: SourceType::Linguist,
    name: "Less",
    extensions: &["less"],
    media_types: &["text/css"],
    signatures: &[],
    related_formats: &[],
};
