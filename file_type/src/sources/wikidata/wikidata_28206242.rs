use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206242: FileFormat = FileFormat {
    id: 28_206_242,
    puid: "wikidata/28206242",
    name: "GX1",
    extensions: &["gx1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFA, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
