use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865563: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_563,
        source_type: SourceType::Wikidata,
        name: "Windows Performance Monitor Alert",
        extensions: &["pma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDC, 0x05, 0x83, 0x40, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
