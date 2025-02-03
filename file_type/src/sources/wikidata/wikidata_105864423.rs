use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864423: FileFormat = FileFormat {
    id: 105_864_423,
    source_type: SourceType::Wikidata,
    name: "Protected Disk Image format",
    extensions: &["pdi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x44, 0x49, 0x2D, 0x4D, 0x53, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
