use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851433: FileFormat = FileFormat {
    id: 105_851_433,
    source_type: SourceType::Wikidata,
    name: "TuneUp Styler Logo Animation",
    extensions: &["tla"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
