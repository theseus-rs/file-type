use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849778: FileFormat = FileFormat {
    id: 105_849_778,
    puid: "wikidata/105849778",
    name: "ColdFusion Component (with rem)",
    extensions: &["cfc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2A, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
