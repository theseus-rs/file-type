use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_7520656: FileType = FileType {
    file_format: &FileFormat {
        id: 7_520_656,
        source_type: SourceType::Wikidata,
        name: "Simple Data Format",
        extensions: &["sdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x44, 0x46, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
