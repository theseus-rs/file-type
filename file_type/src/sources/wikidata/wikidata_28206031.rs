use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206031: FileFormat = FileFormat {
    id: 28_206_031,
    puid: "wikidata/28206031",
    name: "EggPaint",
    extensions: &["trp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x52, 0x55, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
