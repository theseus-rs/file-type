use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852802: FileFormat = FileFormat {
    id: 105_852_802,
    puid: "wikidata/105852802",
    name: "Aegis Videoscape 3D Set",
    extensions: &["set"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x53, 0x31, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
