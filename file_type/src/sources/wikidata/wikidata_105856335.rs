use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856335: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_335,
        source_type: SourceType::Wikidata,
        name: "Fusion Dialog Development System Dialog",
        extensions: &["dds"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x44, 0x44, 0x53, 0x2F, 0x53, 0x44, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
