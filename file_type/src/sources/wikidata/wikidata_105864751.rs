use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864751: FileFormat = FileFormat {
    id: 105_864_751,
    source_type: SourceType::Wikidata,
    name: "Point Cloud Data",
    extensions: &["pcd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x2E, 0x50, 0x43, 0x44, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
