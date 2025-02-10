use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3612023431: FileType = FileType {
    file_format: &FileFormat {
        id: 3_612_023_431,
        source_type: SourceType::Iana,
        name: "vnd.oci.image.manifest.v1+json",
        extensions: &[],
        media_types: &["application/vnd.oci.image.manifest.v1+json"],
        signatures: &[],
        related_formats: &[],
    },
};
