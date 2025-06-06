use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853786: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_786,
        source_type: SourceType::Wikidata,
        name: "K+K Research TAC compressed audio",
        extensions: &["tst"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x74, 0x73, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
