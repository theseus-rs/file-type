use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967387: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_387,
        source_type: SourceType::Wikidata,
        name: "AdLib instrument bank",
        extensions: &["bnk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x44, 0x4C, 0x49, 0x42, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
