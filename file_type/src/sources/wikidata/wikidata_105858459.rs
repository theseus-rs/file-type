use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858459: FileFormat = FileFormat {
    id: 105_858_459,
    source_type: SourceType::Wikidata,
    name: "IAR Embedded Workbench Debug info",
    extensions: &["ewd"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
