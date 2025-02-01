use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856778: FileFormat = FileFormat {
    id: 105_856_778,
    puid: "wikidata/105856778",
    name: "Ultra Fractal data and pixels",
    extensions: &["ufr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x6C, 0x74, 0x72, 0x61, 0x46, 0x72, 0x61, 0x63, 0x74, 0x61, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
