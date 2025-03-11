use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865105: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_105,
        source_type: SourceType::Wikidata,
        name: "PFS Plan spreadsheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6C, 0x61, 0x6E, 0x20, 0x20, 0x01, 0x00, 0x03, 0x41, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
