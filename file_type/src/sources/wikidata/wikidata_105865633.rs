use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865633: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_633,
        source_type: SourceType::Wikidata,
        name: "BitRock installation Package",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x69, 0x74, 0x52, 0x6F, 0x63, 0x6B, 0x50, 0x61, 0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
