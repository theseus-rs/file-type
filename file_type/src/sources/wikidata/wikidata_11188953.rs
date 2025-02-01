use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_11188953: FileFormat = FileFormat {
    id: 11_188_953,
    puid: "wikidata/11188953",
    name: "Astrotite",
    extensions: &["afa"],
    media_types: &["application/x-astrotite-afa"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x53, 0x54, 0x56, 0x53, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
