use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854749: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_749,
        source_type: SourceType::Wikidata,
        name: "ARRIRAW image",
        extensions: &["ari"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x52, 0x52, 0x49, 0x12, 0x34, 0x56, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
