use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858222: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_222,
        source_type: SourceType::Wikidata,
        name: "Preferred Executable Format (PowerPC)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x6F, 0x79, 0x21, 0x70, 0x65, 0x66, 0x66, 0x70, 0x77, 0x70, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
