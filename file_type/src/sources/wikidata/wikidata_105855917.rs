use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855917: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_917,
        source_type: SourceType::Wikidata,
        name: "Vermont Database Corporation database",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x40, 0x44, 0x42, 0x20, 0x56, 0x28])],
                },
            }],
        }],
        related_formats: &[],
    },
};
