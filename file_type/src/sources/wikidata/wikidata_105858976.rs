use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858976: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_976,
        source_type: SourceType::Wikidata,
        name: "GrafX2 bitmap",
        extensions: &["pkm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
