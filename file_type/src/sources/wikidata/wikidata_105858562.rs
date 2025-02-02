use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858562: FileFormat = FileFormat {
    id: 105_858_562,
    source_type: SourceType::Wikidata,
    name: "Dynamix Bitmap data container",
    extensions: &["blk", "bmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0x50, 0x3A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0x50, 0x3A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
