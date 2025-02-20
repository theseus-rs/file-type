use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856544: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_544,
        source_type: SourceType::Wikidata,
        name: "WordStar document (gen)",
        extensions: &["doc", "ws"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
