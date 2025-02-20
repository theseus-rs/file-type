use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859733: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_733,
        source_type: SourceType::Wikidata,
        name: "Brute Force and Ignorance video",
        extensions: &["bfi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x46, 0x26, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
