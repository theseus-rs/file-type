use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858648: FileFormat = FileFormat {
    id: 105_858_648,
    puid: "wikidata/105858648",
    name: "Hitachi Raster Format bitmap",
    extensions: &["hrf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x44, 0x43, 0x2F, 0x4B, 0x52, 0x20, 0x52, 0x53, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
