use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851880: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_880,
        source_type: SourceType::Wikidata,
        name: "Snzip compressed (framing2 format)",
        extensions: &["sz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x06, 0x00, 0x00, 0x73, 0x4E, 0x61, 0x50, 0x70, 0x59,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
