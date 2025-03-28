use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854584: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_584,
        source_type: SourceType::Wikidata,
        name: "DB/TextWorks Database Access Control",
        extensions: &["acf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x43, 0x46, 0x20, 0x30, 0x30, 0x32, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
