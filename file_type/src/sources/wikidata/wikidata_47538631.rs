use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538631: FileFormat = FileFormat {
    id: 47_538_631,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Custom Dictionary",
    extensions: &["cus"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
