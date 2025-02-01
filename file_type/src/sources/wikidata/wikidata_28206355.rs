use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206355: FileFormat = FileFormat {
    id: 28_206_355,
    puid: "wikidata/28206355",
    name: "InShape IIM",
    extensions: &["iim"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x5F, 0x49, 0x4D, 0x41, 0x47, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
