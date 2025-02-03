use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860972: FileFormat = FileFormat {
    id: 105_860_972,
    source_type: SourceType::Wikidata,
    name: "P-CAD ASCII Library",
    extensions: &["lia"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x41, 0x44, 0x5F, 0x41, 0x53, 0x43, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
