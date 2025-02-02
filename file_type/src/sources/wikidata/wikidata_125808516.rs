use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125808516: FileFormat = FileFormat {
    id: 125_808_516,
    source_type: SourceType::Wikidata,
    name: "Mnemosyne Flash-card Collection",
    extensions: &["mem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
