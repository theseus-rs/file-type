use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857371: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_371,
        source_type: SourceType::Wikidata,
        name: "BigJig Jigsaw",
        extensions: &["jg6"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x47, 0x4A, 0x49, 0x47, 0x36, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
