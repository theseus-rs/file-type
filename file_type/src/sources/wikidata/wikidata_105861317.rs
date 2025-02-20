use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861317: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_317,
        source_type: SourceType::Wikidata,
        name: "Cinema 4D Preset Library",
        extensions: &["lib4d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x62, 0x66, 0x68])],
                },
            }],
        }],
        related_formats: &[],
    },
};
