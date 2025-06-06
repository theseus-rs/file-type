use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856641: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_641,
        source_type: SourceType::Wikidata,
        name: "Weapon definition script",
        extensions: &["weap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x44, 0x65, 0x66,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
