use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967395: FileFormat = FileFormat {
    id: 27_967_395,
    source_type: SourceType::Wikidata,
    name: "Adlib Tracker II module",
    extensions: &["a2m"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x41, 0x32, 0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
