use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859250: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_250,
        source_type: SourceType::Wikidata,
        name: "GTA: San Andreas save game (v2.00 PC)",
        extensions: &["b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF6, 0x8D, 0x14, 0xFD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
