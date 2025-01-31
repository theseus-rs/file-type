use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866937: FileFormat = FileFormat {
    id: 105_866_937,
    puid: "wikidata/105866937",
    name: "3DVIA Virtools behavioral object",
    extensions: &["nmo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x65, 0x6D, 0x6F, 0x20, 0x46, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
