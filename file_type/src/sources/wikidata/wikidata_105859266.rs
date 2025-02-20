use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859266: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_266,
        source_type: SourceType::Wikidata,
        name: "GTA: San Andreas save game (v2 PS2)",
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
