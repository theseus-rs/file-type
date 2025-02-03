use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856156: FileFormat = FileFormat {
    id: 105_856_156,
    source_type: SourceType::Wikidata,
    name: "DiskDupe 5.12 disk image",
    extensions: &["ddi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x18, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
