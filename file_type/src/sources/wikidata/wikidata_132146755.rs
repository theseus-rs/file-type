use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_132146755: FileFormat = FileFormat {
    id: 132_146_755,
    source_type: SourceType::Wikidata,
    name: "BrailleBlaster ZIP File",
    extensions: &["bbz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
