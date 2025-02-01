use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863962: FileFormat = FileFormat {
    id: 105_863_962,
    puid: "wikidata/105863962",
    name: "Crytek XML Material",
    extensions: &["mtl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x20, 0x4D, 0x74, 0x6C,
                    0x46, 0x6C, 0x61, 0x67, 0x73, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
