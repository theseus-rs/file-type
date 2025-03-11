use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861637: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_637,
        source_type: SourceType::Wikidata,
        name: "LUKS encrypted",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x55, 0x4B, 0x53, 0xBA, 0xBE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
