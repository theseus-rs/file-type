use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861881: FileFormat = FileFormat {
    id: 105_861_881,
    source_type: SourceType::Wikidata,
    name: "Informative Graphics Markup",
    extensions: &["mrk"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x48, 0x44, 0x52, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
