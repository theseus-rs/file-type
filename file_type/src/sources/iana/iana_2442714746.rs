use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2442714746: FileType = FileType {
    file_format: &FileFormat {
        id: 2_442_714_746,
        source_type: SourceType::Iana,
        name: "iges",
        extensions: &[],
        media_types: &["application/iges"],
        signatures: &[],
        related_formats: &[],
    },
};
