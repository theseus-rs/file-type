use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850268: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_268,
        source_type: SourceType::Wikidata,
        name: "Compact compressed data (alt)",
        extensions: &["c"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
