use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851319: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_319,
        source_type: SourceType::Wikidata,
        name: "TrID definition! :-)",
        extensions: &["xml"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x54, 0x72, 0x49, 0x44, 0x20, 0x76, 0x65, 0x72, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
