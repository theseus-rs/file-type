use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856645: FileFormat = FileFormat {
    id: 105_856_645,
    puid: "wikidata/105856645",
    name: "Verity Collection Index Descriptor",
    extensions: &["wld"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
