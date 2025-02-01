use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856087: FileFormat = FileFormat {
    id: 105_856_087,
    puid: "wikidata/105856087",
    name: "Digifont Outline Font Description",
    extensions: &["dfi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF1, 0x29])],
            },
        }],
    }],
    related_formats: &[],
};
