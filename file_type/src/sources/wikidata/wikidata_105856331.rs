use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_331,
        source_type: SourceType::Wikidata,
        name: "Lotus Works Document",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAE, 0x26, 0x56, 0x30, 0x30, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
