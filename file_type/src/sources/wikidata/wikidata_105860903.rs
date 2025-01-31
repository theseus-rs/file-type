use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860903: FileFormat = FileFormat {
    id: 105_860_903,
    puid: "wikidata/105860903",
    name: "MicroStation Symbology Resources",
    extensions: &["rsc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x53, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                    0x53, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x20, 0x52, 0x65, 0x73,
                    0x6F, 0x75, 0x72, 0x63, 0x65, 0x73, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
