use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862373: FileFormat = FileFormat {
    id: 105_862_373,
    source_type: SourceType::Wikidata,
    name: "MegaZeux game",
    extensions: &["mzx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
