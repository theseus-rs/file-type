use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121543321: FileFormat = FileFormat {
    id: 121_543_321,
    source_type: SourceType::Wikidata,
    name: "TaxCut 2008 Tax Return File",
    extensions: &["t08"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
