use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206485: FileFormat = FileFormat {
    id: 28_206_485,
    source_type: SourceType::Wikidata,
    name: "Khronos Texture",
    extensions: &["ktx"],
    media_types: &["image/ktx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
