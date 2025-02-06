use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61912820: FileFormat = FileFormat {
    id: 61_912_820,
    source_type: SourceType::Wikidata,
    name: "ODM",
    extensions: &["odm"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
