use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857400: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_400,
        source_type: SourceType::Wikidata,
        name: "Java serialized data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0xED])],
                },
            }],
        }],
        related_formats: &[],
    },
};
