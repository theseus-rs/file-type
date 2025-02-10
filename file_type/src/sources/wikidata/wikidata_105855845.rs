use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855845: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_845,
        source_type: SourceType::Wikidata,
        name: "Call of Duty 4 map",
        extensions: &["d3dbsp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x42, 0x53, 0x50, 0x16, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
