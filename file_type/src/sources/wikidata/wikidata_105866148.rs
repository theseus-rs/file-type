use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866148: FileFormat = FileFormat {
    id: 105_866_148,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Pivot (gen)",
    extensions: &["pvt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x50, 0x56, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
