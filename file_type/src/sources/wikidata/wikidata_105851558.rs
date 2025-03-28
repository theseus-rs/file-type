use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851558: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_558,
        source_type: SourceType::Wikidata,
        name: "Aladdin 4D TXList",
        extensions: &["txl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x42, 0x4D, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
