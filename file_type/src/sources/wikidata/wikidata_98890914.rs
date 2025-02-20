use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_98890914: FileType = FileType {
    file_format: &FileFormat {
        id: 98_890_914,
        source_type: SourceType::Wikidata,
        name: "Windows Calendar",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xB5, 0xA2, 0xB0, 0xB3, 0xB3, 0xB0, 0xA2, 0xB5,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
