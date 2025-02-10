use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850645: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_645,
        source_type: SourceType::Wikidata,
        name: "Knowledge Master Concept Map",
        extensions: &["kmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x57, 0x04, 0x01, 0x31, 0x12])],
                },
            }],
        }],
        related_formats: &[],
    },
};
