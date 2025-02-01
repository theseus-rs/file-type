use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851537: FileFormat = FileFormat {
    id: 105_851_537,
    puid: "wikidata/105851537",
    name: "TFM Music Maker music (V2)",
    extensions: &["tfe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x46, 0x4D, 0x66, 0x6D, 0x74, 0x56, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
