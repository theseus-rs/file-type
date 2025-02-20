use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2816237978: FileType = FileType {
    file_format: &FileFormat {
        id: 2_816_237_978,
        source_type: SourceType::Iana,
        name: "prc",
        extensions: &[],
        media_types: &["model/prc"],
        signatures: &[],
        related_formats: &[],
    },
};
