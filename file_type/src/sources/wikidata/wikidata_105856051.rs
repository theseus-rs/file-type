use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856051: FileFormat = FileFormat {
    id: 105_856_051,
    puid: "wikidata/105856051",
    name: "Oric disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x52, 0x49, 0x43, 0x44, 0x49, 0x53, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
