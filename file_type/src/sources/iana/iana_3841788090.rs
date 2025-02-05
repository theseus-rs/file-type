use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3841788090: FileFormat = FileFormat {
    id: 3_841_788_090,
    source_type: SourceType::Iana,
    name: "vnd.arastra.swi (OBSOLETED in favor of application/vnd.aristanetworks.swi)",
    extensions: &[],
    media_types: &["application/vnd.arastra.swi"],
    signatures: &[],
    related_formats: &[],
};
