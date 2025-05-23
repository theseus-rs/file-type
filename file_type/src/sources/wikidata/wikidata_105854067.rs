use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854067: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_067,
        source_type: SourceType::Wikidata,
        name: "paq8o8 compressed archive",
        extensions: &["paq8o8"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x61, 0x71, 0x38, 0x6F, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
