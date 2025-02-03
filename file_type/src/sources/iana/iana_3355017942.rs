use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3355017942: FileFormat = FileFormat {
    id: 3_355_017_942,
    source_type: SourceType::Iana,
    name: "midi-clip",
    extensions: &[],
    media_types: &["audio/midi-clip"],
    internal_signatures: &[],
    related_formats: &[],
};
