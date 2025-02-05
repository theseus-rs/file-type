use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851126: FileFormat = FileFormat {
    id: 105_851_126,
    source_type: SourceType::Wikidata,
    name: "Timed Text Markup Language (UTF-8)",
    extensions: &["ttml"],
    media_types: &["application/ttml+xml"],
    signatures: &[Signature {
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
