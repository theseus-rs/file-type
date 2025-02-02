use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853653: FileFormat = FileFormat {
    id: 105_853_653,
    source_type: SourceType::Wikidata,
    name: "ABC notation (old)",
    extensions: &["abc"],
    media_types: &["text/vnd.abc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
