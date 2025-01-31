use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850311: FileFormat = FileFormat {
    id: 105_850_311,
    puid: "wikidata/105850311",
    name: "Javelin Case study",
    extensions: &["cas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF7, 0x12, 0x34, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
