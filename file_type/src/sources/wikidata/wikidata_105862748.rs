use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_748,
        source_type: SourceType::Wikidata,
        name: "MovieShop machine definition",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x41, 0x4D, 0x45, 0x3A, 0x20, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
