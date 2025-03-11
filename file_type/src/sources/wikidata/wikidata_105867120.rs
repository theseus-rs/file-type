use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867120: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_120,
        source_type: SourceType::Wikidata,
        name: "NNCP compressed data (v1)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB7, 0x27, 0xAC, 0x57, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
