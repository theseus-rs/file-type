use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_912766839: FileType = FileType {
    file_format: &FileFormat {
        id: 912_766_839,
        source_type: SourceType::Iana,
        name: "G7291",
        extensions: &[],
        media_types: &["audio/G7291"],
        signatures: &[],
        related_formats: &[],
    },
};
