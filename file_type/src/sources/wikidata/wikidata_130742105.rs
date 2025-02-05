use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130742105: FileFormat = FileFormat {
    id: 130_742_105,
    source_type: SourceType::Wikidata,
    name: "Scaml markup file",
    extensions: &["scaml"],
    media_types: &["text/x-scaml"],
    signatures: &[],
    related_formats: &[],
};
