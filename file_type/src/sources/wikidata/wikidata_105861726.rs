use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861726: FileFormat = FileFormat {
    id: 105_861_726,
    source_type: SourceType::Wikidata,
    name: "Quartus Memory Initialization File (with rem)",
    extensions: &["mif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x2D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
