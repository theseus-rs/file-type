use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855631: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_631,
        source_type: SourceType::Wikidata,
        name: "i8086 relocatable Object code",
        extensions: &["o"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x52, 0x65, 0x6C, 0x6F, 0x63, 0x29, 0x00, 0x69, 0x38, 0x30, 0x38,
                        0x36,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
