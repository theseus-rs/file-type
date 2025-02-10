use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_751800: FileType = FileType {
    file_format: &FileFormat {
        id: 751_800,
        source_type: SourceType::Wikidata,
        name: "TrueType Font",
        extensions: &["dfont", "tte", "ttf"],
        media_types: &["font/ttf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
