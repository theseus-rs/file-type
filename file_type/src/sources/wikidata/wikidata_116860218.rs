use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116860218: FileFormat = FileFormat {
    id: 116_860_218,
    source_type: SourceType::Wikidata,
    name: "Forms Maker & Filler Forms file",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
