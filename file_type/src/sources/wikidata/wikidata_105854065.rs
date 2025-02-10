use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854065: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_065,
        source_type: SourceType::Wikidata,
        name: "VOCPACK lossless compressed audio",
        extensions: &["vp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x46, 0x56, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
