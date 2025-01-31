use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865849: FileFormat = FileFormat {
    id: 105_865_849,
    puid: "wikidata/105865849",
    name: "MediaShow Production (v1.0)",
    extensions: &["prod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x48, 0x4F, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
