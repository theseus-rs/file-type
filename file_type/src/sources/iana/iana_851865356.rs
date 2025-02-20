use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_851865356: FileType = FileType {
    file_format: &FileFormat {
        id: 851_865_356,
        source_type: SourceType::Iana,
        name: "vnd.mdl",
        extensions: &[],
        media_types: &["application/vnd.mdl"],
        signatures: &[],
        related_formats: &[],
    },
};
