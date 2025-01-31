use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855415: FileFormat = FileFormat {
    id: 105_855_415,
    puid: "wikidata/105855415",
    name: "PageRender3D facet/object",
    extensions: &["facet"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x61, 0x63, 0x65, 0x74, 0x20, 0x20, 0x20, 0x63, 0x61, 0x72, 0x74, 0x20,
                    0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
