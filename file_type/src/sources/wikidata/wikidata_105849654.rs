use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849654: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_654,
        source_type: SourceType::Wikidata,
        name: "Cura variant configuration",
        extensions: &["cfg"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x6C, 0x5D, 0x0A, 0x6E, 0x61,
                        0x6D, 0x65, 0x20, 0x3D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
