use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_573500267: FileType = FileType {
    file_format: &FileFormat {
        id: 573_500_267,
        source_type: SourceType::Iana,
        name: "parityfec",
        extensions: &[],
        media_types: &["audio/parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
