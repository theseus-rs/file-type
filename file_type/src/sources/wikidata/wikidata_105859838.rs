use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859838: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_838,
        source_type: SourceType::Wikidata,
        name: "FreeMotion Flash movie",
        extensions: &["sqf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x00, 0x00, 0x00, 0x47, 0x4D, 0x6F, 0x76, 0x69, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
