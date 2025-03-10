use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762727: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_727,
        source_type: SourceType::Wikidata,
        name: "Ability document",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x2D, 0x48, 0x44, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
