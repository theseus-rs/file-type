use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854275: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_275,
        source_type: SourceType::Wikidata,
        name: "Seattle FilmWorks / PhotoWorks photo album",
        extensions: &["alb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x5F, 0x41, 0x4C, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
