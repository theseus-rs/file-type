use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850254: FileFormat = FileFormat {
    id: 105_850_254,
    puid: "wikidata/105850254",
    name: "PMarc CP/M SFX archive",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x70, 0x6D, 0x73, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
