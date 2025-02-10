use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862046: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_046,
        source_type: SourceType::Wikidata,
        name: "Aladdin 4D MTR",
        extensions: &["mtr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x50, 0x4D, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
