use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860190: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_190,
        source_type: SourceType::Wikidata,
        name: "Mario Kart Wii ghost data",
        extensions: &["rkg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x4B, 0x47, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
