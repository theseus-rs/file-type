use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855875: FileFormat = FileFormat {
    id: 105_855_875,
    puid: "wikidata/105855875",
    name: "Psion Archive Data Base",
    extensions: &["dbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x72, 0x6D, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
