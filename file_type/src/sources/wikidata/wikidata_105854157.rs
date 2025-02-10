use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854157: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_157,
        source_type: SourceType::Wikidata,
        name: "ar archive (thin)",
        extensions: &["a", "ar", "lbr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x3C, 0x74, 0x68, 0x69, 0x6E, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
