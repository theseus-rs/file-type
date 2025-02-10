use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851222: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_222,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Template (v2.x)",
        extensions: &["tpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x70, 0x6C, 0x74, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
