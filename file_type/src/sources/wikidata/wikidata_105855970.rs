use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855970: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_970,
        source_type: SourceType::Wikidata,
        name: "Dyalog APL Session",
        extensions: &["dse"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAA, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
