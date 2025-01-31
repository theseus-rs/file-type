use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853696: FileFormat = FileFormat {
    id: 105_853_696,
    puid: "wikidata/105853696",
    name: "CAZIPXP compressed archive",
    extensions: &["caz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
