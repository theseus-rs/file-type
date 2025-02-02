use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1228770: FileFormat = FileFormat {
    id: 1_228_770,
    source_type: SourceType::Wikidata,
    name: "Disk Masher System",
    extensions: &["dms"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4D, 0x53, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
