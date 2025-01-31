use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651336: FileFormat = FileFormat {
    id: 29_651_336,
    puid: "wikidata/29651336",
    name: "PmDraw",
    extensions: &["pmd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x57, 0x20, 0x52, 0x46, 0x57, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
