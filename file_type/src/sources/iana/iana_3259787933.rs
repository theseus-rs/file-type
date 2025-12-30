use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3259787933: FileType = FileType {
    file_format: &FileFormat {
        id: 3_259_787_933,
        source_type: SourceType::Iana,
        name: "scitt-receipt+cose",
        extensions: &[],
        media_types: &["application/scitt-receipt+cose"],
        signatures: &[],
        related_formats: &[],
    },
};
