use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1228770: FileFormat = FileFormat {
    id: 1_228_770,
    source_type: SourceType::Wikidata,
    name: "Disk Masher System",
    extensions: &["dms"],
    media_types: &[],
    signatures: &[Signature {
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
