use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853252: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_252,
        source_type: SourceType::Wikidata,
        name: "Roland Fantom audio",
        extensions: &["svd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x6E, 0x53, 0x56, 0x44, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
