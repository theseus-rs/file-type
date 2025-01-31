use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853508: FileFormat = FileFormat {
    id: 105_853_508,
    puid: "wikidata/105853508",
    name: "ZSNES movie capture",
    extensions: &["zmv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x4D, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
