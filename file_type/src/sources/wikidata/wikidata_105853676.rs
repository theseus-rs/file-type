use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853676: FileFormat = FileFormat {
    id: 105_853_676,
    puid: "wikidata/105853676",
    name: "AOS File Format",
    extensions: &["aos"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
