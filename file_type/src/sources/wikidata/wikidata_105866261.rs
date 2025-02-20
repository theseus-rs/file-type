use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866261: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_261,
        source_type: SourceType::Wikidata,
        name: "PipeDream document",
        extensions: &["pd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x4F, 0x50, 0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
