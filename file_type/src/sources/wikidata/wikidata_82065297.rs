use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_82065297: FileFormat = FileFormat {
    id: 82_065_297,
    puid: "wikidata/82065297",
    name: "Micrografx Media Manager Easy Catalog",
    extensions: &["ecf"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x61, 0x69, 0x6E, 0x5D, 0x0D, 0x0A, 0x4E, 0x61, 0x6D, 0x65, 0x3D,
                    0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
