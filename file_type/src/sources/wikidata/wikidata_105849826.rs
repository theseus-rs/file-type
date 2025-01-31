use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849826: FileFormat = FileFormat {
    id: 105_849_826,
    puid: "wikidata/105849826",
    name: "Chile compressed file",
    extensions: &["chl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x4C, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
