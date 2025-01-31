use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861988: FileFormat = FileFormat {
    id: 105_861_988,
    puid: "wikidata/105861988",
    name: "TommySoftware CAD/Draw drawing (v1)",
    extensions: &["mpg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x6F, 0x6D, 0x6D, 0x79, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
                    0x20, 0x4D, 0x50, 0x47, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
