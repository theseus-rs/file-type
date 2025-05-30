use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762724: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_724,
        source_type: SourceType::Wikidata,
        name: "PAM Dataset",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x50, 0x41, 0x4D, 0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
