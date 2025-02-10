use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_614,
        source_type: SourceType::Wikidata,
        name: "ZSoft Palette",
        extensions: &["pal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0B, 0x00, 0x5A, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
