use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762794: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_794,
        source_type: SourceType::Wikidata,
        name: "XML Localization Interchange File Format (UTF-16 LE)",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFE, 0x3C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
