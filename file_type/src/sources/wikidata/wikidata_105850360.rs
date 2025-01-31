use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850360: FileFormat = FileFormat {
    id: 105_850_360,
    puid: "wikidata/105850360",
    name: "Chem3D model",
    extensions: &["c3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x63, 0x4D, 0x41, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
