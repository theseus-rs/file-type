use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864392: FileFormat = FileFormat {
    id: 105_864_392,
    puid: "wikidata/105864392",
    name: "PDI Disk Image (Type 1)",
    extensions: &["pdi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x44, 0x49, 0x54, 0x59, 0x50, 0x45, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
