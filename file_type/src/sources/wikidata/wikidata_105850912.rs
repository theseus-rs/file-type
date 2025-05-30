use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850912: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_912,
        source_type: SourceType::Wikidata,
        name: "Mario Kart Wii course description",
        extensions: &["kmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x4B, 0x4D, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
