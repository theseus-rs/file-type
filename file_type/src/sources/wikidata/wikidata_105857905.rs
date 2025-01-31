use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857905: FileFormat = FileFormat {
    id: 105_857_905,
    puid: "wikidata/105857905",
    name: "MasterDOS v2.x bootable disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x13, 0x4D, 0x44, 0x4F, 0x53, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
