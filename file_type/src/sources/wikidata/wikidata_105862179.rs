use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862179: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_179,
        source_type: SourceType::Wikidata,
        name: "Multiple Sequence File (nucleic acid)",
        extensions: &["msf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x21, 0x4E, 0x41, 0x5F, 0x4D, 0x55, 0x4C, 0x54, 0x49, 0x50, 0x4C,
                        0x45, 0x5F, 0x41, 0x4C, 0x49, 0x47, 0x4E, 0x4D, 0x45, 0x4E, 0x54, 0x20,
                        0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
