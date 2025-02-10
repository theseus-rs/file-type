use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865738: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_738,
        source_type: SourceType::Wikidata,
        name: "HyperPAD Pad",
        extensions: &["pad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x50, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
