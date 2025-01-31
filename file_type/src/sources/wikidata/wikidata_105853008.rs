use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853008: FileFormat = FileFormat {
    id: 105_853_008,
    puid: "wikidata/105853008",
    name: "Symbian Series 3 Installation file",
    extensions: &["sisx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7A, 0x1A, 0x20, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
