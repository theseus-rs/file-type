use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851126: FileFormat = FileFormat {
    id: 105_851_126,
    puid: "wikidata/105851126",
    name: "Timed Text Markup Language (UTF-8)",
    extensions: &["ttml"],
    media_types: &["application/ttml+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
