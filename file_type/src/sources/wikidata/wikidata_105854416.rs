use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854416: FileFormat = FileFormat {
    id: 105_854_416,
    puid: "wikidata/105854416",
    name: "Amiga Money data (v1) (generic)",
    extensions: &["amm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x4D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
