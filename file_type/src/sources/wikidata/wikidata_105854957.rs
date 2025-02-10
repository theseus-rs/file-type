use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854957: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_957,
        source_type: SourceType::Wikidata,
        name: "NaShrinK compressed archive",
        extensions: &["nsk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x53, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
