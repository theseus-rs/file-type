use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857599: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_599,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Item (v1.1)",
        extensions: &["itm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x54, 0x4D, 0x20, 0x56, 0x31, 0x2E, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
