use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849893: FileFormat = FileFormat {
    id: 105_849_893,
    puid: "wikidata/105849893",
    name: "Harvard Graphics Chart (vA.01)",
    extensions: &["cht"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x4F, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
