use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859503: FileFormat = FileFormat {
    id: 105_859_503,
    source_type: SourceType::Wikidata,
    name: "Dalet Volume info",
    extensions: &["vol"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE5, 0xC5])],
            },
        }],
    }],
    related_formats: &[],
};
