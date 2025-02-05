use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132146755: FileFormat = FileFormat {
    id: 132_146_755,
    source_type: SourceType::Wikidata,
    name: "BrailleBlaster ZIP File",
    extensions: &["bbz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
