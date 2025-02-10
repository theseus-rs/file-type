use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860462: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_462,
        source_type: SourceType::Wikidata,
        name: "R saved work space",
        extensions: &["rdata"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x44, 0x58, 0x32, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
