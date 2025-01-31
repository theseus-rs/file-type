use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866288: FileFormat = FileFormat {
    id: 105_866_288,
    puid: "wikidata/105866288",
    name: "Pksmart compressed data (v1)",
    extensions: &["pks"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x53, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
