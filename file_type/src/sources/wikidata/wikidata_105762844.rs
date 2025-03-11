use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762844: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_844,
        source_type: SourceType::Wikidata,
        name: "DarkCryptTC XDC encrypted container",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x44, 0x43, 0x4E, 0x45, 0x58, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
