use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857134: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_134,
        source_type: SourceType::Wikidata,
        name: "Mario Party 5 game data",
        extensions: &["hsf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x53, 0x46, 0x56, 0x30, 0x33, 0x37])],
                },
            }],
        }],
        related_formats: &[],
    },
};
