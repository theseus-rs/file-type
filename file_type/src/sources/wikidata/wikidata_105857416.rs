use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857416: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_416,
        source_type: SourceType::Wikidata,
        name: "JCALG1 compressed data",
        extensions: &["jc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
