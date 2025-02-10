use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849791: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_791,
        source_type: SourceType::Wikidata,
        name: "Cereal Encrypted File",
        extensions: &["cef"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x65, 0x72, 0x65, 0x61, 0x6C, 0x20, 0x45, 0x6E, 0x63, 0x72, 0x79,
                        0x70, 0x74, 0x65, 0x64, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
