use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_674,
        source_type: SourceType::Wikidata,
        name: "CTable2 datum grid shift format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x54, 0x41, 0x42, 0x4C, 0x45, 0x20, 0x56, 0x32, 0x2E, 0x30, 0x20,
                        0x20, 0x20, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
