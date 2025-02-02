use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866136: FileFormat = FileFormat {
    id: 105_866_136,
    source_type: SourceType::Wikidata,
    name: "QNX Photon Font (bitmap)",
    extensions: &["phf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x51, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
