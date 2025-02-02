use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71831258: FileFormat = FileFormat {
    id: 71_831_258,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 2",
    extensions: &["cdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x4C, 0x6D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
