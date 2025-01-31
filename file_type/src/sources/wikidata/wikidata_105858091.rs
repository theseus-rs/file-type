use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858091: FileFormat = FileFormat {
    id: 105_858_091,
    puid: "wikidata/105858091",
    name: "InDesign Shortcuts set",
    extensions: &["indk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x50, 0x50, 0x50, 0x43, 0x53, 0x42, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
