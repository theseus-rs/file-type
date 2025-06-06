use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861959: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_959,
        source_type: SourceType::Wikidata,
        name: "Bochs keymap",
        extensions: &["map"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x42, 0x6F, 0x63, 0x68, 0x73, 0x20, 0x4B, 0x65, 0x79, 0x6D,
                        0x61, 0x70, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
