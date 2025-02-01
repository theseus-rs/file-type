use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866291: FileFormat = FileFormat {
    id: 105_866_291,
    puid: "wikidata/105866291",
    name: "Polyfilm Preferences",
    extensions: &["prf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x6C, 0x79, 0x5F, 0x50, 0x72, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
