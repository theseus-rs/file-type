use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853888: FileFormat = FileFormat {
    id: 105_853_888,
    source_type: SourceType::Wikidata,
    name: "Covox ADPCM encoded audio",
    extensions: &["cvx", "v2s", "v3s", "v4s", "v8", "vmf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x55, 0xFF, 0xAA, 0xFF, 0x55, 0xFF, 0xAA,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
