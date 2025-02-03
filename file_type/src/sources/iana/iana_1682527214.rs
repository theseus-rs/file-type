use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1682527214: FileFormat = FileFormat {
    id: 1_682_527_214,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
