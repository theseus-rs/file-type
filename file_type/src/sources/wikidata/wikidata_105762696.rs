use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762696: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_696,
        source_type: SourceType::Wikidata,
        name: "Disney Animation Studio eXposure Sheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x50, 0x4F, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
