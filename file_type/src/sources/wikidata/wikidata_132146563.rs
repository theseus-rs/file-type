use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_132146563: FileFormat = FileFormat {
    id: 132_146_563,
    source_type: SourceType::Wikidata,
    name: "BrailleBlaster XML File",
    extensions: &["bbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
