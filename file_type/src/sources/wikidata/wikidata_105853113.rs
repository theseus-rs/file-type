use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_113,
        source_type: SourceType::Wikidata,
        name: "Samna Word document",
        extensions: &["sm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x53, 0x00, 0x00, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
