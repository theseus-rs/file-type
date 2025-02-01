use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207521: FileFormat = FileFormat {
    id: 28_207_521,
    puid: "wikidata/28207521",
    name: "WPB",
    extensions: &["wpb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x50, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
