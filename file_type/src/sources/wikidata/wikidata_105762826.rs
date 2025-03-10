use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762826: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_826,
        source_type: SourceType::Wikidata,
        name: "XML 2D graphics",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x78, 0x32, 0x64, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
