use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855069: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_069,
        source_type: SourceType::Wikidata,
        name: "AZZ Cardfile index",
        extensions: &["~i"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x7E, 0x41, 0x5A, 0x5A, 0x20, 0x43, 0x41, 0x52, 0x44, 0x46, 0x49,
                        0x4C, 0x45, 0x20, 0x41, 0x5A, 0x5A, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20,
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x7E, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
