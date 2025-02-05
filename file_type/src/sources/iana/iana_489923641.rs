use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_489923641: FileFormat = FileFormat {
    id: 489_923_641,
    source_type: SourceType::Iana,
    name: "vnd.syncml.dmddf+wbxml",
    extensions: &[],
    media_types: &["application/vnd.syncml.dmddf+wbxml"],
    signatures: &[],
    related_formats: &[],
};
