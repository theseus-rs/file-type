use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_198: FileType = FileType {
    file_format: &FileFormat {
        id: 198,
        source_type: SourceType::Linguist,
        name: "Less",
        extensions: &["less"],
        media_types: &["text/x-less"],
        signatures: &[],
        related_formats: &[],
    },
};
