use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860673: FileFormat = FileFormat {
    id: 105_860_673,
    source_type: SourceType::Wikidata,
    name: "RealTime subtitles",
    extensions: &["rt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
