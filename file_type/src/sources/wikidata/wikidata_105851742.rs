use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851742: FileFormat = FileFormat {
    id: 105_851_742,
    puid: "wikidata/105851742",
    name: "Aegis VideoSEG Script",
    extensions: &["script"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
