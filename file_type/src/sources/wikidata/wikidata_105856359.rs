use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856359: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_359,
        source_type: SourceType::Wikidata,
        name: "ThinManager configuration (v7)",
        extensions: &["db"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x68, 0x69, 0x6E, 0x4D, 0x61, 0x6E, 0x61, 0x67, 0x65, 0x72, 0x20,
                        0x37,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
