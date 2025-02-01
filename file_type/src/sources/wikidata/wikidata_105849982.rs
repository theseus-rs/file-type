use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849982: FileFormat = FileFormat {
    id: 105_849_982,
    puid: "wikidata/105849982",
    name: "Turbo Pascal 3.0 Chain module",
    extensions: &["chn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE8, 0xFE, 0xDD])],
            },
        }],
    }],
    related_formats: &[],
};
