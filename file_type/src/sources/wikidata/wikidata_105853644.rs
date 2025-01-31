use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853644: FileFormat = FileFormat {
    id: 105_853_644,
    puid: "wikidata/105853644",
    name: "Cybiko Application",
    extensions: &["app"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x79])],
            },
        }],
    }],
    related_formats: &[],
};
