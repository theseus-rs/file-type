use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4060300356: FileFormat = FileFormat {
    id: 4_060_300_356,
    source_type: SourceType::Iana,
    name: "vnd.nato.bindingdataobject+cbor",
    extensions: &[],
    media_types: &["application/vnd.nato.bindingdataobject+cbor"],
    signatures: &[],
    related_formats: &[],
};
