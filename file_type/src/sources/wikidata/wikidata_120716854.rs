use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120716854: FileFormat = FileFormat {
    id: 120_716_854,
    source_type: SourceType::Wikidata,
    name: "TaxCut 2006 Tax Return file",
    extensions: &["t06"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
