use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856870: FileFormat = FileFormat {
    id: 105_856_870,
    puid: "wikidata/105856870",
    name: "NTv2 Standard ASCII Grid Shift",
    extensions: &["gsa"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x55, 0x4D, 0x5F, 0x4F, 0x52, 0x45, 0x43, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
