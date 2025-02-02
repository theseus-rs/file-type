use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855803: FileFormat = FileFormat {
    id: 105_855_803,
    source_type: SourceType::Wikidata,
    name: "Dream X2 Preset Format",
    extensions: &["dxp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB9, 0xF6, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
