use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857862: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_862,
        source_type: SourceType::Wikidata,
        name: "ISI gMotor SRW type game data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x52, 0x57, 0x30, 0x30, 0x30, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
