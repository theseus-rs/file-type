use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762698: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_698,
        source_type: SourceType::Wikidata,
        name: "XPEC Entertainment game Archive Format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x41, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
