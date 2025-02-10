use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_331,
        source_type: SourceType::Wikidata,
        name: "JPC-RR rerecording",
        extensions: &["jrsr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x52, 0x53, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
