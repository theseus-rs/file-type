use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859749: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_749,
        source_type: SourceType::Wikidata,
        name: "Psygnosis YOP video",
        extensions: &["yop"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x4F, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
