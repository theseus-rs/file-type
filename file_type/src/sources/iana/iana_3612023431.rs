use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3612023431: FileFormat = FileFormat {
    id: 3_612_023_431,
    source_type: SourceType::Iana,
    name: "vnd.oci.image.manifest.v1+json",
    extensions: &[],
    media_types: &["application/vnd.oci.image.manifest.v1+json"],
    internal_signatures: &[],
    related_formats: &[],
};
