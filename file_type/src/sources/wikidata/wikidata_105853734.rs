use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853734: FileFormat = FileFormat {
    id: 105_853_734,
    puid: "wikidata/105853734",
    name: "Quest Adventure Script (generic)",
    extensions: &["asl"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x27, 0x20, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
