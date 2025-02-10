use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_2190356: FileType = FileType {
    file_format: &FileFormat {
        id: 2_190_356,
        source_type: SourceType::Wikidata,
        name: "JPEG-LS",
        extensions: &["jls"],
        media_types: &["image/jls"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xD8, 0xFF, 0xF7, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
