use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854133: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_133,
        source_type: SourceType::Wikidata,
        name: "ZyXEL Voice Format audio",
        extensions: &["ad2", "zvd", "zyx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x79, 0x58, 0x45, 0x4C, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
