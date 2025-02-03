use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860279: FileFormat = FileFormat {
    id: 105_860_279,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Polygons (gen)",
    extensions: &["ply"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x50, 0x4C, 0x59])],
            },
        }],
    }],
    related_formats: &[],
};
