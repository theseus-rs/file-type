use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862571: FileFormat = FileFormat {
    id: 105_862_571,
    source_type: SourceType::Wikidata,
    name: "Poser Material (V5)",
    extensions: &["mt5"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
