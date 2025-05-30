use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859567: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_567,
        source_type: SourceType::Wikidata,
        name: "Vice Flip List",
        extensions: &["vfl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x56, 0x69, 0x63, 0x65, 0x20, 0x66, 0x6C, 0x69, 0x70, 0x6C,
                        0x69, 0x73, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
