use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_041,
        source_type: SourceType::Wikidata,
        name: "MSX BASIC Graphics bitmap (screen 7-8-12)",
        extensions: &["sc7", "sc8", "scc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x00, 0xFF, 0xD3, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
