use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_725,
        source_type: SourceType::Wikidata,
        name: "Casio Model 9860 add-in",
        extensions: &["g1a"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xAA, 0xAC, 0xBD, 0xAF, 0x90, 0x88, 0x9A, 0x8D, 0x0C, 0xFF, 0xEF, 0xFF,
                        0xEF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
