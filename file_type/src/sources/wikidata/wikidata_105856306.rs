use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856306: FileFormat = FileFormat {
    id: 105_856_306,
    puid: "wikidata/105856306",
    name: "Skype chatsynch (old)",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x43, 0x64, 0x42, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
