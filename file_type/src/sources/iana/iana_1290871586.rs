use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1290871586: FileType = FileType {
    file_format: &FileFormat {
        id: 1_290_871_586,
        source_type: SourceType::Iana,
        name: "opus",
        extensions: &[],
        media_types: &["audio/opus"],
        signatures: &[],
        related_formats: &[],
    },
};
