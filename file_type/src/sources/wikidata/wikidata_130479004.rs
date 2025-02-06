use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130479004: FileFormat = FileFormat {
    id: 130_479_004,
    source_type: SourceType::Wikidata,
    name: "Pointless source code file",
    extensions: &["ptls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
