use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852634: FileFormat = FileFormat {
    id: 105_852_634,
    source_type: SourceType::Wikidata,
    name: "NGS orbital format SP3 (with velocity)",
    extensions: &["sp3"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x61, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
