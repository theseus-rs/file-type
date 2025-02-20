use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_208976687: FileType = FileType {
    file_format: &FileFormat {
        id: 208_976_687,
        source_type: SourceType::Linguist,
        name: "Sieve",
        extensions: &["sieve"],
        media_types: &["application/sieve"],
        signatures: &[],
        related_formats: &[],
    },
};
