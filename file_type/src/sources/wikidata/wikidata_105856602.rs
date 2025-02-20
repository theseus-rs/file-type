use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856602: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_602,
        source_type: SourceType::Wikidata,
        name: "MUST music / song",
        extensions: &["wvz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x56, 0x5A, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
