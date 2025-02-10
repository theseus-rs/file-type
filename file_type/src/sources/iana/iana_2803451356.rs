use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2803451356: FileType = FileType {
    file_format: &FileFormat {
        id: 2_803_451_356,
        source_type: SourceType::Iana,
        name: "ATFX",
        extensions: &[],
        media_types: &["application/ATFX"],
        signatures: &[],
        related_formats: &[],
    },
};
