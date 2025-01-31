use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850044: FileFormat = FileFormat {
    id: 105_850_044,
    puid: "wikidata/105850044",
    name: "Harvard Graphics Chart (v3.x)",
    extensions: &["ch3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
