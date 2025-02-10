use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856810: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_810,
        source_type: SourceType::Wikidata,
        name: "IBM Graphing Assistant Graph",
        extensions: &["gra"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x72, 0x61, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
