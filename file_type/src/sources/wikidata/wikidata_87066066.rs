use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87066066: FileFormat = FileFormat {
    id: 87_066_066,
    puid: "wikidata/87066066",
    name: "LEADToolsCompressed Image",
    extensions: &["cmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x45, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
