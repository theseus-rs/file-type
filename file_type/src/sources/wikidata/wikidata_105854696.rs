use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854696: FileFormat = FileFormat {
    id: 105_854_696,
    source_type: SourceType::Wikidata,
    name: "Acclaim Skeleton File",
    extensions: &["asf"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23])],
            },
        }],
    }],
    related_formats: &[],
};
