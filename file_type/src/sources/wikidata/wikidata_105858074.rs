use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858074: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_074,
        source_type: SourceType::Wikidata,
        name: "OS/2 INF",
        extensions: &["inf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x53, 0x50, 0x01, 0x9B, 0x00, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
