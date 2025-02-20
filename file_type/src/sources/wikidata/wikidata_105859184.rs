use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859184: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_184,
        source_type: SourceType::Wikidata,
        name: "Nintendo GameCube/Wii 3D Model (ASCII)",
        extensions: &["bdl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x33, 0x44, 0x32, 0x62, 0x64, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
