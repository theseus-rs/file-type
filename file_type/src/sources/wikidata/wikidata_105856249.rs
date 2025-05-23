use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_249,
        source_type: SourceType::Wikidata,
        name: "Delusion samples library",
        extensions: &["del"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x45, 0x4C, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
