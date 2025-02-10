use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967294: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_294,
        source_type: SourceType::Wikidata,
        name: "Musink project",
        extensions: &["musink"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
