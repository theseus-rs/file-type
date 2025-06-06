use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856123: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_123,
        source_type: SourceType::Wikidata,
        name: "Weresc CADE drawing",
        extensions: &["dtc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0C, 0x1E, 0x43, 0x41, 0x44, 0x45, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
                        0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
