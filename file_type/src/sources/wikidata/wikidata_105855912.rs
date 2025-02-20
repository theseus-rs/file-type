use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855912: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_912,
        source_type: SourceType::Wikidata,
        name: "DESI-III drawing",
        extensions: &["bin", "din"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x45, 0x53, 0x49, 0x2D, 0x49, 0x49, 0x49, 0x2D, 0x42, 0x49, 0x4E,
                        0x2D, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
