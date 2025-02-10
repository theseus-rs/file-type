use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856856: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_856,
        source_type: SourceType::Wikidata,
        name: "Bars and Pipes Guitar Player Chords bank",
        extensions: &["gchord"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4C, 0x55, 0x45, 0x52, 0x49, 0x42, 0x42, 0x4F, 0x4E, 0x47, 0x43,
                        0x48, 0x4F, 0x52, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
