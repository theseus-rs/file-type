use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850141: FileFormat = FileFormat {
    id: 105_850_141,
    puid: "wikidata/105850141",
    name: "CHITUBOX profile Configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x5F, 0x72, 0x65, 0x73, 0x69, 0x6E, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
