use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853194: FileFormat = FileFormat {
    id: 105_853_194,
    puid: "wikidata/105853194",
    name: "Korg Song file",
    extensions: &["sng"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x4F, 0x52, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
