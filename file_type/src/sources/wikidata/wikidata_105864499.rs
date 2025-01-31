use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864499: FileFormat = FileFormat {
    id: 105_864_499,
    puid: "wikidata/105864499",
    name: "Ultimo Primo SnapShot",
    extensions: &["pss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
