use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120717288: FileFormat = FileFormat {
    id: 120_717_288,
    source_type: SourceType::Wikidata,
    name: "TaxCut 2007 Tax Return file",
    extensions: &["t07"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
