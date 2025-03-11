use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849870: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_870,
        source_type: SourceType::Wikidata,
        name: "Compo preferences",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x20, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72,
                        0x65, 0x6E, 0x63, 0x65, 0x73, 0x0A, 0x53, 0x61, 0x76, 0x65, 0x64, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
