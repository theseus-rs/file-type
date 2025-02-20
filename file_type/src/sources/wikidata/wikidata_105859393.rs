use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859393: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_393,
        source_type: SourceType::Wikidata,
        name: "Zelda Classic Quest (Unencoded)",
        extensions: &["qsu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x47, 0x20, 0x5A, 0x43, 0x20, 0x45, 0x6E, 0x68, 0x61, 0x6E, 0x63,
                        0x65, 0x64, 0x20, 0x51, 0x75, 0x65, 0x73, 0x74, 0x20, 0x46, 0x69, 0x6C,
                        0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
