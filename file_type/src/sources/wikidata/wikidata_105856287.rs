use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_287,
        source_type: SourceType::Wikidata,
        name: "Virtuoso Display Resource File",
        extensions: &["drf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x72, 0x44, 0x65, 0x66, 0x69, 0x6E, 0x65, 0x44, 0x69, 0x73, 0x70,
                        0x6C, 0x61, 0x79, 0x28,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
