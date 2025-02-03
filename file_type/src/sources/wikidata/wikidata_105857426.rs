use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857426: FileFormat = FileFormat {
    id: 105_857_426,
    source_type: SourceType::Wikidata,
    name: "PALASM (var.1)",
    extensions: &["jed"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x50, 0x41, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
