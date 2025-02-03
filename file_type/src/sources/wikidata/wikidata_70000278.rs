use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_70000278: FileFormat = FileFormat {
    id: 70_000_278,
    source_type: SourceType::Wikidata,
    name: "FAMILYFILE",
    extensions: &["familyfile"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
