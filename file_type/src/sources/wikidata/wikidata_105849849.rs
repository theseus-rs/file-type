use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849849: FileFormat = FileFormat {
    id: 105_849_849,
    source_type: SourceType::Wikidata,
    name: "CXF Vector Map Format (v1.x)",
    extensions: &["cxf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x58, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
