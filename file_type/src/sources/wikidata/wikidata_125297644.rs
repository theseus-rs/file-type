use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125297644: FileFormat = FileFormat {
    id: 125_297_644,
    source_type: SourceType::Wikidata,
    name: "Scheme library source",
    extensions: &["sls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
