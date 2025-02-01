use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859696: FileFormat = FileFormat {
    id: 105_859_696,
    puid: "wikidata/105859696",
    name: "Ventura Publisher Graphics bitmap",
    extensions: &["vgr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x00, 0x4C, 0x4A])],
            },
        }],
    }],
    related_formats: &[],
};
