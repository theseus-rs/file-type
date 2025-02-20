use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959844: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_844,
        source_type: SourceType::Wikidata,
        name: "Jeskola Buzz song",
        extensions: &["bmw", "bmx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x75, 0x7A, 0x7A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
