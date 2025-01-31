use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853836: FileFormat = FileFormat {
    id: 105_853_836,
    puid: "wikidata/105853836",
    name: "Dar archive",
    extensions: &["dar"],
    media_types: &["application/x-dar"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
