use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856912: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_912,
        source_type: SourceType::Wikidata,
        name: "Airline Tycoon game data archive",
        extensions: &["gli", "glj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4C, 0x49, 0x42, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
