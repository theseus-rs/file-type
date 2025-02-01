use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854727: FileFormat = FileFormat {
    id: 105_854_727,
    puid: "wikidata/105854727",
    name: "ESP - Extension Sort Packer compressed archive",
    extensions: &["esp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x53, 0x50, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
