use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762868: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_868,
        source_type: SourceType::Wikidata,
        name: "XNA Framework Content Pipeline Binary (Win Phone 7)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x4E, 0x42, 0x6D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
