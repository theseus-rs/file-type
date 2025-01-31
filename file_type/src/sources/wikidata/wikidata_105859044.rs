use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859044: FileFormat = FileFormat {
    id: 105_859_044,
    puid: "wikidata/105859044",
    name: "Plane Minimizing Bitmap Compression bitmap",
    extensions: &["pmbc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x42, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
