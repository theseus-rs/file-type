use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858490: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_490,
        source_type: SourceType::Wikidata,
        name: "BrioQuery",
        extensions: &["bqy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x42, 0x52, 0x49, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
