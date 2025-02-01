use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857594: FileFormat = FileFormat {
    id: 105_857_594,
    puid: "wikidata/105857594",
    name: "B-DOS bootable disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x13, 0x42, 0x2D, 0x44, 0x4F, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
