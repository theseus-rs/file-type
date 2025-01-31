use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851920: FileFormat = FileFormat {
    id: 105_851_920,
    puid: "wikidata/105851920",
    name: "VIA setup configuration file",
    extensions: &["scf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x43, 0x46, 0x5D, 0x0D, 0x0A, 0x43, 0x4F, 0x4D, 0x50, 0x41, 0x4E,
                    0x59, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
