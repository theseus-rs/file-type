use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862849: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_849,
        source_type: SourceType::Wikidata,
        name: "MouseWrite document (v2.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x57, 0x04, 0x00, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
