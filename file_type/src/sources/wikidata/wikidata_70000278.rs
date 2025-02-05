use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_70000278: FileFormat = FileFormat {
    id: 70_000_278,
    source_type: SourceType::Wikidata,
    name: "FAMILYFILE",
    extensions: &["familyfile"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
