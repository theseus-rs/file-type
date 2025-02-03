use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858712: FileFormat = FileFormat {
    id: 105_858_712,
    source_type: SourceType::Wikidata,
    name: "DeskPic bitmap",
    extensions: &["gfb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x46, 0x32, 0x35, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
