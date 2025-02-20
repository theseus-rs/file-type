use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2357210: FileType = FileType {
    file_format: &FileFormat {
        id: 2_357_210,
        source_type: SourceType::Wikidata,
        name: "Structured Fax File",
        extensions: &["sff"],
        media_types: &["image/x-sff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x66, 0x66, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
