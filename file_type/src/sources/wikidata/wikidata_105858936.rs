use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858936: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_936,
        source_type: SourceType::Wikidata,
        name: "Koala Paint (C64) bitmap",
        extensions: &["koa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x60])],
                },
            }],
        }],
        related_formats: &[],
    },
};
