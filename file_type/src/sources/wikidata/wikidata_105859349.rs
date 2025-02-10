use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859349: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_349,
        source_type: SourceType::Wikidata,
        name: "3ds Quad colors",
        extensions: &["qop"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x20, 0x51, 0x75, 0x61, 0x64, 0x20, 0x43, 0x6F, 0x6C, 0x6F,
                        0x72, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
