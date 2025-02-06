use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125808516: FileFormat = FileFormat {
    id: 125_808_516,
    source_type: SourceType::Wikidata,
    name: "Mnemosyne Flash-card Collection",
    extensions: &["mem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
