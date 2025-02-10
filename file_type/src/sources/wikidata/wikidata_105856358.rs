use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_358,
        source_type: SourceType::Wikidata,
        name: "DB/TextWorks Database",
        extensions: &["dbr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x42, 0x52, 0x20, 0x30, 0x30, 0x31, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
