use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_287,
        source_type: SourceType::Wikidata,
        name: "Rich Game format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x47, 0x4D, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
