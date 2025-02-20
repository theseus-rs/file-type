use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_606,
        source_type: SourceType::Wikidata,
        name: "AIX Small indexed archive (AIX prior to v4.3)",
        extensions: &["ar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x61, 0x69, 0x61, 0x66, 0x66, 0x3E, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
