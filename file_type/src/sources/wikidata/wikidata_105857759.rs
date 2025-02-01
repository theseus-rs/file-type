use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857759: FileFormat = FileFormat {
    id: 105_857_759,
    puid: "wikidata/105857759",
    name: "Art Icons Pro - IconCollection",
    extensions: &["icc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x43, 0x43, 0x30, 0x01, 0x00, 0x00, 0x00, 0x49, 0x43, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
