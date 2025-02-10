use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862475: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_475,
        source_type: SourceType::Wikidata,
        name: "Metasequoia 3D scene",
        extensions: &["mqo"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x65, 0x74, 0x61, 0x73, 0x65, 0x71, 0x75, 0x6F, 0x69, 0x61, 0x20,
                        0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
