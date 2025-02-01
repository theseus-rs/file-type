use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854021: FileFormat = FileFormat {
    id: 105_854_021,
    puid: "wikidata/105854021",
    name: "AWeb Plugin",
    extensions: &["awebplugin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x03, 0xF3])],
            },
        }],
    }],
    related_formats: &[],
};
