use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207178: FileFormat = FileFormat {
    id: 28_207_178,
    puid: "wikidata/28207178",
    name: "Q0 Image Attributes",
    extensions: &["fal"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x5F, 0x41, 0x4C, 0x4C, 0x28, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
