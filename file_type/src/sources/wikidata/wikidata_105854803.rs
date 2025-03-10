use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854803: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_803,
        source_type: SourceType::Wikidata,
        name: "Crunch compressed archive",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
