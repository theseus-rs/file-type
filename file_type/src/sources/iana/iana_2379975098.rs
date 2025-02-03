use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2379975098: FileFormat = FileFormat {
    id: 2_379_975_098,
    source_type: SourceType::Iana,
    name: "vnd.ipld.dag-cbor",
    extensions: &[],
    media_types: &["application/vnd.ipld.dag-cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
