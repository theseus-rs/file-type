use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853349: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_349,
        source_type: SourceType::Wikidata,
        name: "Sega Master System/Game Gear/Coleco Music Format (v1.0)",
        extensions: &["sgc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x47, 0x43, 0x1A, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
