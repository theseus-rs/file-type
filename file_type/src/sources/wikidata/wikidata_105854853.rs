use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854853: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_853,
        source_type: SourceType::Wikidata,
        name: "Eschalon Setup ARCV compressed container",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x52, 0x43, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
