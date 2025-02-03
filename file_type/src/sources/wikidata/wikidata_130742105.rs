use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130742105: FileFormat = FileFormat {
    id: 130_742_105,
    source_type: SourceType::Wikidata,
    name: "Scaml markup file",
    extensions: &["scaml"],
    media_types: &["text/x-scaml"],
    internal_signatures: &[],
    related_formats: &[],
};
