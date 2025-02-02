use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538631: FileFormat = FileFormat {
    id: 47_538_631,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Custom Dictionary",
    extensions: &["cus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
