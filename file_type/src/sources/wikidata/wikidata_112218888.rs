use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_112218888: FileType = FileType {
    file_format: &FileFormat {
        id: 112_218_888,
        source_type: SourceType::Wikidata,
        name: "Adobe Audition Peak File",
        extensions: &["pkf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x24, 0x21, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
