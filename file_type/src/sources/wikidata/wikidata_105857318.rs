use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857318: FileFormat = FileFormat {
    id: 105_857_318,
    source_type: SourceType::Wikidata,
    name: "L3DT compressed Heightfield Format",
    extensions: &["hf2"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x46, 0x32, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
