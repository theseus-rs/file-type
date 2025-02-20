use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_245,
        source_type: SourceType::Wikidata,
        name: "Squeak package",
        extensions: &["sar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x22, 0x53, 0x71, 0x75, 0x65, 0x61, 0x6B, 0x20, 0x61, 0x72, 0x63, 0x68,
                        0x69, 0x76, 0x65, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
