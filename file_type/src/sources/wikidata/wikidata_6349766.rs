use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_6349766: FileType = FileType {
    file_format: &FileFormat {
        id: 6_349_766,
        source_type: SourceType::Wikidata,
        name: "VCDIFF",
        extensions: &["vcdiff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD6, 0xC3, 0xC4])],
                },
            }],
        }],
        related_formats: &[],
    },
};
