use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864229: FileFormat = FileFormat {
    id: 105_864_229,
    puid: "wikidata/105864229",
    name: "SpeedScript document (C64)",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
