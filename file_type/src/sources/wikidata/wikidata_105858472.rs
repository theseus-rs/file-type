use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858472: FileFormat = FileFormat {
    id: 105_858_472,
    source_type: SourceType::Wikidata,
    name: "easyHDR 2 Settings",
    extensions: &["ehsx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
