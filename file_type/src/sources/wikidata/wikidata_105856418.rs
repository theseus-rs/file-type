use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856418: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_418,
        source_type: SourceType::Wikidata,
        name: "COSMI FormsMaker Form (v2.0)",
        extensions: &["wfm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x73, 0x6D, 0x69, 0x46, 0x6F, 0x72, 0x6D, 0x73, 0x4D, 0x61,
                        0x6B, 0x65, 0x72, 0x56, 0x65, 0x72, 0x2E, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
