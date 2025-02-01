use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853183: FileFormat = FileFormat {
    id: 105_853_183,
    puid: "wikidata/105853183",
    name: "SFZ Sample definition (with rem)",
    extensions: &["sfz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
