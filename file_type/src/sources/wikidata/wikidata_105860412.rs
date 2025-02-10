use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860412: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_412,
        source_type: SourceType::Wikidata,
        name: "REALbasic Project",
        extensions: &["rbvcp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x79, 0x70, 0x65, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
