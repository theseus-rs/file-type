use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858965: FileFormat = FileFormat {
    id: 105_858_965,
    source_type: SourceType::Wikidata,
    name: "Naive Image format NII animated bitmaps",
    extensions: &["nii"],
    media_types: &["application/octet-stream", "image/nii"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0xC3, 0xAF, 0x49])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0xC3, 0xAF, 0x49])],
                },
            }],
        },
    ],
    related_formats: &[],
};
