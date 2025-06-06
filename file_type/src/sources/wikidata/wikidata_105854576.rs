use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854576: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_576,
        source_type: SourceType::Wikidata,
        name: "Generated application MS Visual FoxPro 7",
        extensions: &["app"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xF2])],
                },
            }],
        }],
        related_formats: &[],
    },
};
