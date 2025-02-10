use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_198: FileType = FileType {
    file_format: &FileFormat {
        id: 198,
        source_type: SourceType::Linguist,
        name: "Less",
        extensions: &["less"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
