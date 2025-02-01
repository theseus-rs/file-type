use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960028: FileFormat = FileFormat {
    id: 27_960_028,
    puid: "wikidata/27960028",
    name: "VocPack",
    extensions: &["vp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x46, 0x56, 0x50, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
