use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859290: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_290,
        source_type: SourceType::Wikidata,
        name: "Balance of Power: The 1990 Ed. saved game",
        extensions: &["bop"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4F, 0x50, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
