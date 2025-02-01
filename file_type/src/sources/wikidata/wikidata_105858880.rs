use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858880: FileFormat = FileFormat {
    id: 105_858_880,
    puid: "wikidata/105858880",
    name: "Grayscale BitMap",
    extensions: &["gbm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x42, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
