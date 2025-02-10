use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_124840545: FileType = FileType {
    file_format: &FileFormat {
        id: 124_840_545,
        source_type: SourceType::Wikidata,
        name: "Toon Boom Project",
        extensions: &["tbp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x6F, 0x6F, 0x6E, 0x42, 0x6F, 0x6F, 0x6D, 0x54, 0x65, 0x63, 0x68,
                        0x6E, 0x6F, 0x6C, 0x6F, 0x67, 0x69, 0x65, 0x73, 0x49, 0x6E, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
