use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_606,
        source_type: SourceType::Wikidata,
        name: "EVE Online data (generic)",
        extensions: &["blue"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x6C, 0x75, 0x65, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
