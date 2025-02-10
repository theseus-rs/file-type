use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4050793986: FileType = FileType {
    file_format: &FileFormat {
        id: 4_050_793_986,
        source_type: SourceType::Iana,
        name: "vnd.smintio.portals.archive",
        extensions: &[],
        media_types: &["application/vnd.smintio.portals.archive"],
        signatures: &[],
        related_formats: &[],
    },
};
