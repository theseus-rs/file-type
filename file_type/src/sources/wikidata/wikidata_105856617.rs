use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856617: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_617,
        source_type: SourceType::Wikidata,
        name: "Wondershare Video Editor project (XML)",
        extensions: &["wve"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x54, 0x69, 0x6D, 0x65, 0x4C, 0x69, 0x6E, 0x65, 0x50, 0x72, 0x6F,
                        0x67, 0x72, 0x61, 0x6D, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
