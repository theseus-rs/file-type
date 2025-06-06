use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862597: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_597,
        source_type: SourceType::Wikidata,
        name: "Multiplan spreadsheet (v2.x)",
        extensions: &["mod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0C, 0xEC, 0x00, 0x00, 0x08, 0xAB, 0x08, 0x00, 0x1F, 0x00, 0x1A, 0x00,
                        0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
