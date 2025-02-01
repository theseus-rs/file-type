use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858064: FileFormat = FileFormat {
    id: 105_858_064,
    puid: "wikidata/105858064",
    name: "Picasa info (generic)",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x69, 0x63, 0x61, 0x73, 0x61, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
