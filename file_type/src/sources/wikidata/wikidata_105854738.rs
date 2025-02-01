use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854738: FileFormat = FileFormat {
    id: 105_854_738,
    puid: "wikidata/105854738",
    name: "Amiga Money reports (v1)",
    extensions: &["amm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4D, 0x31, 0x52, 0x45, 0x50, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
