use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867537: FileFormat = FileFormat {
    id: 105_867_537,
    puid: "wikidata/105867537",
    name: "Open Minecraft Note Block Studio Song (v4)",
    extensions: &["nbs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x04, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
