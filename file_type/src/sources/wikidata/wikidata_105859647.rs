use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859647: FileFormat = FileFormat {
    id: 105_859_647,
    source_type: SourceType::Wikidata,
    name: "Nintendo 3DS Flipnote Studio 3D video",
    extensions: &["kwc", "kwz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x46, 0x48])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x46, 0x48])],
                },
            }],
        },
    ],
    related_formats: &[],
};
