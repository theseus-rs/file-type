use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1107365668: FileType = FileType {
    file_format: &FileFormat {
        id: 1_107_365_668,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.spreadsheet",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};
