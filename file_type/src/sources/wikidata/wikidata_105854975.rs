use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854975: FileFormat = FileFormat {
    id: 105_854_975,
    source_type: SourceType::Wikidata,
    name: "YBS compressed archive",
    extensions: &["ybs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x42, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
