use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_154,
        source_type: SourceType::Wikidata,
        name: "Swift 3D 3D Graphic",
        extensions: &["t3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x65, 0x67, 0x69, 0x6E, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
