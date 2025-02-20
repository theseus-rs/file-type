use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855767: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_767,
        source_type: SourceType::Wikidata,
        name: "Qt Assistant Content File",
        extensions: &["dcf"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x44, 0x43,
                        0x46, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
