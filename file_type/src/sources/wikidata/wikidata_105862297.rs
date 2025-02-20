use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862297: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_297,
        source_type: SourceType::Wikidata,
        name: "Multiplan spreadsheet (v4.x)",
        extensions: &["mod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x09, 0x00, 0x04, 0x00, 0x40, 0x01, 0x10, 0x00, 0x42, 0x00, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
