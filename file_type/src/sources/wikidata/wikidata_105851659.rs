use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851659: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_659,
        source_type: SourceType::Wikidata,
        name: "Legend Style",
        extensions: &["sty"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xEF, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
