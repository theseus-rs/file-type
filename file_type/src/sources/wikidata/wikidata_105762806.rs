use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762806: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_806,
        source_type: SourceType::Wikidata,
        name: "XFDL form",
        extensions: &[],
        media_types: &["application/vnd.xfdl"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
