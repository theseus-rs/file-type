use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_15838827: FileFormat = FileFormat {
    id: 15_838_827,
    puid: "wikidata/15838827",
    name: "TopoJSON",
    extensions: &["topojson"],
    media_types: &["application/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x22, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3A, 0x22, 0x54, 0x6F, 0x70, 0x6F,
                    0x6C, 0x6F, 0x67, 0x79, 0x22, 0x2C, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
