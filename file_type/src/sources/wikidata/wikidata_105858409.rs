use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858409: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_409,
        source_type: SourceType::Wikidata,
        name: "Elastic Reality Shape",
        extensions: &["ers"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x45, 0x6C, 0x61, 0x73, 0x74, 0x69, 0x63, 0x20, 0x52, 0x65, 0x61,
                        0x6C, 0x69, 0x74, 0x79, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
