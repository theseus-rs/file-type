use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445592: FileFormat = FileFormat {
    id: 28_445_592,
    source_type: SourceType::Wikidata,
    name: "AMOS Memory Bank",
    extensions: &["abk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x6D, 0x42, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
