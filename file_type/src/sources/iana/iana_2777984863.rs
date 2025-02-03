use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2777984863: FileFormat = FileFormat {
    id: 2_777_984_863,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.vae-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.vae-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
