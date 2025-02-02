use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125808650: FileFormat = FileFormat {
    id: 125_808_650,
    source_type: SourceType::Wikidata,
    name: "Mnemosyne 2.0 file",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
