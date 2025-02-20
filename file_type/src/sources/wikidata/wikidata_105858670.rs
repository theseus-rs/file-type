use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858670: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_670,
        source_type: SourceType::Wikidata,
        name: "Bugbiter APAC239i bitmap",
        extensions: &["bgp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x55, 0x47, 0x42, 0x49, 0x54, 0x45, 0x52, 0x5F, 0x41, 0x50, 0x41,
                        0x43, 0x32, 0x33, 0x39, 0x49, 0x5F, 0x50, 0x49, 0x43, 0x54, 0x55, 0x52,
                        0x45, 0x5F, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
