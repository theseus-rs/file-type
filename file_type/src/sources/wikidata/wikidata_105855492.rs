use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855492: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_492,
        source_type: SourceType::Wikidata,
        name: "FMOD Designer Project",
        extensions: &["fdp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x3E, 0x0D, 0x0A, 0x3C,
                        0x6E, 0x61, 0x6D, 0x65, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
