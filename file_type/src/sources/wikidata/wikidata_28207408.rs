use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207408: FileFormat = FileFormat {
    id: 28_207_408,
    puid: "wikidata/28207408",
    name: "Utah RLE",
    extensions: &["rle"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0xCC])],
            },
        }],
    }],
    related_formats: &[],
};
