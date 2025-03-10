use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856998: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_998,
        source_type: SourceType::Wikidata,
        name: "Gallery Lock container",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x49, 0x2D, 0x47, 0x41, 0x4C, 0x4C, 0x52, 0x59, 0x2D, 0x4C,
                        0x4F, 0x43, 0x4B, 0x2D, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
