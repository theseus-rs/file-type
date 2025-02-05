use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853286: FileFormat = FileFormat {
    id: 105_853_286,
    source_type: SourceType::Wikidata,
    name: "DVDSubtitle subtitles",
    extensions: &["sub"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x48, 0x45, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
