use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853566: FileFormat = FileFormat {
    id: 105_853_566,
    puid: "wikidata/105853566",
    name: "Speculator '97 snapshot",
    extensions: &["zx82"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x58, 0x38, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
