use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865373: FileFormat = FileFormat {
    id: 105_865_373,
    puid: "wikidata/105865373",
    name: "PCSX2 Patch",
    extensions: &["pnach"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x67, 0x61, 0x6D, 0x65, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
