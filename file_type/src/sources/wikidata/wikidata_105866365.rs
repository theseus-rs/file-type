use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866365: FileFormat = FileFormat {
    id: 105_866_365,
    puid: "wikidata/105866365",
    name: "Paint.NET Image (v3)",
    extensions: &["pdn"],
    media_types: &["image/x-paintnet"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x44, 0x4E, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
