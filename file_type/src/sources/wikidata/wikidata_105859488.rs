use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859488: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_488,
        source_type: SourceType::Wikidata,
        name: "QlikView document",
        extensions: &["qvw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x17, 0x01, 0x00, 0xC1, 0x06, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
