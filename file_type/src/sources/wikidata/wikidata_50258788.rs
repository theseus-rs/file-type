use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50258788: FileFormat = FileFormat {
    id: 50_258_788,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 2013",
    extensions: &["vsdx"],
    media_types: &["application/vnd.ms-visio.drawing.main+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
