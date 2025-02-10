use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858838: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_838,
        source_type: SourceType::Wikidata,
        name: "GOES Satellite bitmap",
        extensions: &["goe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xC8, 0xC4, 0xD9, 0x40, 0xC1, 0xD9, 0xC5, 0xC1, 0x00, 0x00, 0x04, 0xD0,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
