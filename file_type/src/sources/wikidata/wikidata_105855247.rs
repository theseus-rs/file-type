use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855247: FileFormat = FileFormat {
    id: 105_855_247,
    puid: "wikidata/105855247",
    name: "AngelCode Bitmap Font (binary)",
    extensions: &["fnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
