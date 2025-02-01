use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856869: FileFormat = FileFormat {
    id: 105_856_869,
    puid: "wikidata/105856869",
    name: "Gravis UltraSound PnP InterWave patch",
    extensions: &["fff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x46, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
