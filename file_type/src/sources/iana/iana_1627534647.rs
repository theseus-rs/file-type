use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1627534647: FileFormat = FileFormat {
    id: 1_627_534_647,
    source_type: SourceType::Iana,
    name: "vnd.bpf",
    extensions: &[],
    media_types: &["application/vnd.bpf"],
    internal_signatures: &[],
    related_formats: &[],
};
