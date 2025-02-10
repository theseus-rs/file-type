use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858758: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_758,
        source_type: SourceType::Wikidata,
        name: "BlueJ Package",
        extensions: &["bluej"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x42, 0x6C, 0x75, 0x65, 0x4A, 0x20, 0x70, 0x61, 0x63, 0x6B, 0x61,
                        0x67, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
