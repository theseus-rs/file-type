use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860251: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_251,
        source_type: SourceType::Wikidata,
        name: "Sierra Robot Animation",
        extensions: &["rbt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x16, 0x00, 0x53, 0x4F, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
