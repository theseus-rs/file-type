use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856918: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_918,
        source_type: SourceType::Wikidata,
        name: "DeluxePaint Gallery",
        extensions: &["gal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x12, 0x00, 0x00, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
