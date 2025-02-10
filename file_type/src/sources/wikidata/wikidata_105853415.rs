use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853415: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_415,
        source_type: SourceType::Wikidata,
        name: "Butcher Signal",
        extensions: &["signal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAB, 0xCD, 0x98, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
