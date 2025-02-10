use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856202: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_202,
        source_type: SourceType::Wikidata,
        name: "Advanced DB Master data (v3.0)",
        extensions: &["d01"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2C, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
