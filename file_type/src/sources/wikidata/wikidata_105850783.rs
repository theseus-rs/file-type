use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850783: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_783,
        source_type: SourceType::Wikidata,
        name: "KolibriOS Packed",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x50, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
