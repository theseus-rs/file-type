use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861532: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_532,
        source_type: SourceType::Wikidata,
        name: "LZFSE compressed data (raw data)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x62, 0x76, 0x78, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
