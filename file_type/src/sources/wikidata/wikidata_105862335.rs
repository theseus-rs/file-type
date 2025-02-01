use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862335: FileFormat = FileFormat {
    id: 105_862_335,
    puid: "wikidata/105862335",
    name: "dBASE 5.0 Multiple index",
    extensions: &["mdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
