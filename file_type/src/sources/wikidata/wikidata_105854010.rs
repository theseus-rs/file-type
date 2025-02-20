use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854010: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_010,
        source_type: SourceType::Wikidata,
        name: "Midtown Madness 2 game data archive",
        extensions: &["ar"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x41, 0x56, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
