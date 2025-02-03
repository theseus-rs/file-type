use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856845: FileFormat = FileFormat {
    id: 105_856_845,
    source_type: SourceType::Wikidata,
    name: "GenePattern GCT format",
    extensions: &["gct"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
