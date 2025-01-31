use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860169: FileFormat = FileFormat {
    id: 105_860_169,
    puid: "wikidata/105860169",
    name: "RetroPlatform Player archive",
    extensions: &["rp9"],
    media_types: &["application/vnd.cloanto.rp9"],
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
