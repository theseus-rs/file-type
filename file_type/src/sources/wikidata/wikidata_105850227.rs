use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850227: FileFormat = FileFormat {
    id: 105_850_227,
    puid: "wikidata/105850227",
    name: "CocosBuilder exported info",
    extensions: &["ccbi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x69, 0x62, 0x63, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
