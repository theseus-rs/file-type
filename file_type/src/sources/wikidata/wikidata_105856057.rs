use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856057: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_057,
        source_type: SourceType::Wikidata,
        name: "Mac OS X folder information",
        extensions: &["ds_store"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x01, 0x42, 0x75, 0x64, 0x31, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
