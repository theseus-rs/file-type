use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855760: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_760,
        source_type: SourceType::Wikidata,
        name: "Genesis the Third Day Data",
        extensions: &["data"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x65, 0x6E, 0x65, 0x73, 0x69, 0x73, 0x30, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
