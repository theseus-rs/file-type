use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1232099675: FileFormat = FileFormat {
    id: 1_232_099_675,
    source_type: SourceType::Iana,
    name: "vnd.syncml.dmtnds+xml",
    extensions: &[],
    media_types: &["application/vnd.syncml.dmtnds+xml"],
    signatures: &[],
    related_formats: &[],
};
