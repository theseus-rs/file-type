use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866122: FileFormat = FileFormat {
    id: 105_866_122,
    puid: "wikidata/105866122",
    name: "Partially Signed Bitcoin Transaction",
    extensions: &["psbt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x73, 0x62, 0x74, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
