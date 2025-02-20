use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858564: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_564,
        source_type: SourceType::Wikidata,
        name: "GTA: San Andreas save game (v2.00 PC German)",
        extensions: &["b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x22, 0xCC, 0x31, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
