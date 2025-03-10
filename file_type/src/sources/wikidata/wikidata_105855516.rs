use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855516: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_516,
        source_type: SourceType::Wikidata,
        name: "Frostbite Chunks game data format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x42, 0x43, 0x48, 0x55, 0x4E, 0x4B, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
