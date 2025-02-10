use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_657,
        source_type: SourceType::Wikidata,
        name: "Crack Art bitmap (low-res)",
        extensions: &["ca1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
