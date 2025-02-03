use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4255329489: FileFormat = FileFormat {
    id: 4_255_329_489,
    source_type: SourceType::Iana,
    name: "ace-groupcomm+cbor",
    extensions: &[],
    media_types: &["application/ace-groupcomm+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
