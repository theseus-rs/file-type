use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855766: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_766,
        source_type: SourceType::Wikidata,
        name: "SilkRoad image",
        extensions: &["ddj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x4D, 0x58, 0x56, 0x44, 0x44, 0x4A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
