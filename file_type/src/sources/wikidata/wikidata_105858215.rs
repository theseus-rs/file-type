use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858215: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_215,
        source_type: SourceType::Wikidata,
        name: "Everything index",
        extensions: &["db"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x5A, 0x44, 0x42, 0x06, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
