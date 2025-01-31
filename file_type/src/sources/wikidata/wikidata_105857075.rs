use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857075: FileFormat = FileFormat {
    id: 105_857_075,
    puid: "wikidata/105857075",
    name: "Opticks Global light",
    extensions: &["glo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x47, 0x4C, 0x4F, 0x42, 0x41, 0x4C, 0x4C, 0x49, 0x47, 0x48, 0x54, 0x76,
                    0x31, 0x2E, 0x30, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
