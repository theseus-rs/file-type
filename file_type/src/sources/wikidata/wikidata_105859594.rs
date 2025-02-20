use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859594: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_594,
        source_type: SourceType::Wikidata,
        name: "Shrew VPN configuration",
        extensions: &["vpn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
