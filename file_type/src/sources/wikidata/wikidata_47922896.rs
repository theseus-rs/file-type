use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47922896: FileFormat = FileFormat {
    id: 47_922_896,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for Macintosh Document, version 4.0",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x37, 0x00, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
