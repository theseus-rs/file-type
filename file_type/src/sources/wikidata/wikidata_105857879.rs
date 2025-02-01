use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857879: FileFormat = FileFormat {
    id: 105_857_879,
    puid: "wikidata/105857879",
    name: "Picasa album info",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x69, 0x63, 0x61, 0x73, 0x61, 0x5D, 0x0D, 0x0A, 0x6E, 0x61, 0x6D,
                    0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
