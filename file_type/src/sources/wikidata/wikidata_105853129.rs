use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853129: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_129,
        source_type: SourceType::Wikidata,
        name: "SpecBAS package",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x58, 0x50, 0x41, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
