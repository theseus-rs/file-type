use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865694: FileFormat = FileFormat {
    id: 105_865_694,
    source_type: SourceType::Wikidata,
    name: "PerlApp settings",
    extensions: &["perlapp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
