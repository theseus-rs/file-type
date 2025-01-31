use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856849: FileFormat = FileFormat {
    id: 105_856_849,
    puid: "wikidata/105856849",
    name: "NETGEN Constructive Solid Geometry format",
    extensions: &["geo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x6C, 0x67, 0x65, 0x62, 0x72, 0x61, 0x69, 0x63, 0x33, 0x64, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
