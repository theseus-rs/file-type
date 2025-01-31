use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850216: FileFormat = FileFormat {
    id: 105_850_216,
    puid: "wikidata/105850216",
    name: "Midtown Madness 3 data",
    extensions: &["cdds"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC8, 0x44, 0x0F, 0x99])],
            },
        }],
    }],
    related_formats: &[],
};
