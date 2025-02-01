use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857625: FileFormat = FileFormat {
    id: 105_857_625,
    puid: "wikidata/105857625",
    name: "ISo Zipped format",
    extensions: &["isz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x73, 0x5A, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
