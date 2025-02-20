use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862662: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_662,
        source_type: SourceType::Wikidata,
        name: "OGRE Material",
        extensions: &["material"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
