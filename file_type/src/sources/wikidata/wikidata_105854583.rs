use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854583: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_583,
        source_type: SourceType::Wikidata,
        name: "Transform compressed",
        extensions: &["tfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x6F, 0x72, 0x6D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
