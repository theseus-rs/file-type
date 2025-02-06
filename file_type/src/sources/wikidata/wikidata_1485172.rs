use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1485172: FileFormat = FileFormat {
    id: 1_485_172,
    source_type: SourceType::Wikidata,
    name: "GENealogical inDEX",
    extensions: &["gendex.txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
