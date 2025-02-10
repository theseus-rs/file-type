use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856506: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_506,
        source_type: SourceType::Wikidata,
        name: "EXP document",
        extensions: &["wxp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4C, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
