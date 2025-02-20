use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858519: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_519,
        source_type: SourceType::Wikidata,
        name: "ImageKnife Raw bitmap",
        extensions: &["raw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4B, 0x52, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
