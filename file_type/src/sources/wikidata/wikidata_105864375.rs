use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864375: FileFormat = FileFormat {
    id: 105_864_375,
    puid: "wikidata/105864375",
    name: "ParaView state",
    extensions: &["pvsm"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x50, 0x61, 0x72, 0x61, 0x56, 0x69, 0x65, 0x77, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
