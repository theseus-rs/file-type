use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_060,
        source_type: SourceType::Wikidata,
        name: "Trackerpacker 3 Music",
        extensions: &["tp3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x4C, 0x58, 0x5F, 0x54, 0x50, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
