use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851869: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_869,
        source_type: SourceType::Wikidata,
        name: "ShowBiZ project",
        extensions: &["sbz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x10, 0x04, 0x00, 0x00, 0xF1, 0xC3, 0xB6, 0x8A, 0x21, 0x03, 0x03, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
