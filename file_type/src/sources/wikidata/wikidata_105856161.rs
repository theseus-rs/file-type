use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856161: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_161,
        source_type: SourceType::Wikidata,
        name: "Develve data",
        extensions: &["develve"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x25, 0x49, 0x6E, 0x70, 0x75, 0x74, 0x25, 0x3E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
