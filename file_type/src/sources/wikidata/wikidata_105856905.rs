use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856905: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_905,
        source_type: SourceType::Wikidata,
        name: "GUI Design Studio design",
        extensions: &["gui"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x0A, 0x09, 0x43, 0x49, 0x44,
                        0x20, 0x3D, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
