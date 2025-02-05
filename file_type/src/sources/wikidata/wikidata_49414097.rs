use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49414097: FileFormat = FileFormat {
    id: 49_414_097,
    source_type: SourceType::Wikidata,
    name: "CATIA Model, version 4",
    extensions: &["mod", "model"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
