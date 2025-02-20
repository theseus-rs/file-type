use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_249,
        source_type: SourceType::Wikidata,
        name: "Skype Extras Manager log",
        extensions: &["ezlog"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1B, 0xA3, 0xD6, 0x46,
                        0x11, 0xCC, 0xAE, 0x15, 0x13, 0x23, 0xFB, 0x72, 0x92, 0x5E, 0x1D, 0x7E,
                        0x68, 0xA1, 0xF8, 0xBA, 0x75, 0x97, 0x7B, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
