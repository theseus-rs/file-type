use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205523: FileFormat = FileFormat {
    id: 28_205_523,
    puid: "wikidata/28205523",
    name: "ICDRAW Group Icon File",
    extensions: &["ib3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x43, 0x42, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
