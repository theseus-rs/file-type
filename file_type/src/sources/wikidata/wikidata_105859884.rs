use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859884: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_884,
        source_type: SourceType::Wikidata,
        name: "Nokia Saved SMS",
        extensions: &["vmg"],
        media_types: &["text/x-vMessage"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x4D, 0x53, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
