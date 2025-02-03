use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125871478: FileFormat = FileFormat {
    id: 125_871_478,
    source_type: SourceType::Wikidata,
    name: "PechaMaker Format",
    extensions: &["pxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
