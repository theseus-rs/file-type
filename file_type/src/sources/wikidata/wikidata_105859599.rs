use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859599: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_599,
        source_type: SourceType::Wikidata,
        name: "PLC Data",
        extensions: &["vd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4C, 0x43, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C,
                        0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
