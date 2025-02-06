use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125808650: FileFormat = FileFormat {
    id: 125_808_650,
    source_type: SourceType::Wikidata,
    name: "Mnemosyne 2.0 file",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
