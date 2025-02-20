use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3335845233: FileType = FileType {
    file_format: &FileFormat {
        id: 3_335_845_233,
        source_type: SourceType::Iana,
        name: "ODA",
        extensions: &[],
        media_types: &["application/ODA"],
        signatures: &[],
        related_formats: &[],
    },
};
