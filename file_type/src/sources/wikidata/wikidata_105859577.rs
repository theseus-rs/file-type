use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859577: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_577,
        source_type: SourceType::Wikidata,
        name: "VICE Rom Set",
        extensions: &["vrs"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x65, 0x72, 0x6E, 0x61, 0x6C, 0x4E, 0x61, 0x6D, 0x65, 0x3D, 0x22,
                        0x6B, 0x65, 0x72, 0x6E, 0x61, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
