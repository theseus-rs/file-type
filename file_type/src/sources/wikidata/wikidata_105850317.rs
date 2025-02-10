use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850317: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_317,
        source_type: SourceType::Wikidata,
        name: "Tomato ARM config format",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x52, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
