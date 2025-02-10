use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_955665527: FileType = FileType {
    file_format: &FileFormat {
        id: 955_665_527,
        source_type: SourceType::Iana,
        name: "parityfec",
        extensions: &[],
        media_types: &["video/parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
