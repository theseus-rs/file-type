use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855505: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_505,
        source_type: SourceType::Wikidata,
        name: "Final Data r2 data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0xEE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
