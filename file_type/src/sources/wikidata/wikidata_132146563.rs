use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132146563: FileFormat = FileFormat {
    id: 132_146_563,
    source_type: SourceType::Wikidata,
    name: "BrailleBlaster XML File",
    extensions: &["bbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
