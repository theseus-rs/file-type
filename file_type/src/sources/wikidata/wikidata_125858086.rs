use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125858086: FileFormat = FileFormat {
    id: 125_858_086,
    source_type: SourceType::Wikidata,
    name: "Python GUI Source File",
    extensions: &["pyw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
