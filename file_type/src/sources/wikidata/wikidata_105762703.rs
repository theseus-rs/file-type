use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762703: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_703,
        source_type: SourceType::Wikidata,
        name: "XTimeTable Time Table",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x20, 0x78, 0x74, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
