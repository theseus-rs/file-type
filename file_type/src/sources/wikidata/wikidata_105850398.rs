use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850398: FileFormat = FileFormat {
    id: 105_850_398,
    source_type: SourceType::Wikidata,
    name: "CXF Vector Map Format (v3.x)",
    extensions: &["cxf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x58, 0x54, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
