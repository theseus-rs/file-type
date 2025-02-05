use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858459: FileFormat = FileFormat {
    id: 105_858_459,
    source_type: SourceType::Wikidata,
    name: "IAR Embedded Workbench Debug info",
    extensions: &["ewd"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
