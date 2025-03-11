use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762941: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_941,
        source_type: SourceType::Wikidata,
        name: "Hexels preferences",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x48, 0x65, 0x78, 0x65, 0x6C, 0x50, 0x72, 0x65, 0x66, 0x73, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
