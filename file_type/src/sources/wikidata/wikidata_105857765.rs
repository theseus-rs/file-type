use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857765: FileFormat = FileFormat {
    id: 105_857_765,
    puid: "wikidata/105857765",
    name: "UCDOS Input Metod Driver",
    extensions: &["imd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x43, 0x44, 0x4F, 0x53, 0x20, 0x49, 0x4D, 0x44, 0x20, 0x46, 0x49, 0x4C,
                    0x45, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
