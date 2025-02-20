use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850349: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_349,
        source_type: SourceType::Wikidata,
        name: "WhatsApp stored messages",
        extensions: &["crypt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0xEF, 0x23, 0xAE, 0xFF, 0xF9, 0x75, 0x0E, 0xC6, 0xCD, 0x5C, 0xA0,
                        0xA5, 0x9F, 0x2D, 0x35,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
