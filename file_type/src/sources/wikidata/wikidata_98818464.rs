use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_98818464: FileType = FileType {
    file_format: &FileFormat {
        id: 98_818_464,
        source_type: SourceType::Wikidata,
        name: "Ace Stream Transport",
        extensions: &["acelive"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x63, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x54, 0x72, 0x61,
                        0x6E, 0x73, 0x70, 0x6F, 0x72, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
