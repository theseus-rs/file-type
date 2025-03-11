use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762713: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_713,
        source_type: SourceType::Wikidata,
        name: "Xbox 360 Executable",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x45, 0x58, 0x32, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
