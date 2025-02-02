use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855685: FileFormat = FileFormat {
    id: 105_855_685,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Object Group (gen)",
    extensions: &["ogp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x4F, 0x47, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
