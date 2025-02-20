use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856562: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_562,
        source_type: SourceType::Wikidata,
        name: "WordStar 5.5 document",
        extensions: &["doc", "ws5"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
