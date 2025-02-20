use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856292: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_292,
        source_type: SourceType::Wikidata,
        name: "Delta Music module",
        extensions: &["dm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4C, 0x4C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
