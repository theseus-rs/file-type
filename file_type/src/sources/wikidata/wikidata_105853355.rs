use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_355,
        source_type: SourceType::Wikidata,
        name: "CADVANCE 2D symbol",
        extensions: &["sym"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x65, 0x72, 0x73, 0x3A, 0x32, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
