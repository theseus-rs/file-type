use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865082: FileFormat = FileFormat {
    id: 105_865_082,
    puid: "wikidata/105865082",
    name: "PowerPoint Macro-enabled Open XML add-in",
    extensions: &["ppam"],
    media_types: &["application/vnd.ms-powerpoint.addin.macroEnabled.12"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
