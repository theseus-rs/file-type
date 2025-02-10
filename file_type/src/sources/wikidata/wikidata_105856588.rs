use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856588: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_588,
        source_type: SourceType::Wikidata,
        name: "WordStar 7 document",
        extensions: &["doc", "ws7"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
