use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850605: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_605,
        source_type: SourceType::Wikidata,
        name: "CoffeeCup Button Factory button",
        extensions: &["cbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x56, 0x49, 0x45, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
