use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856605: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_605,
        source_type: SourceType::Wikidata,
        name: "Outlook Express addressbook",
        extensions: &["wab"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x9C, 0xCB, 0xCB, 0x8D, 0x13, 0x75, 0xD2, 0x11, 0x91, 0x58, 0x00, 0xC0,
                        0x4F, 0x79, 0x56, 0xA4,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
