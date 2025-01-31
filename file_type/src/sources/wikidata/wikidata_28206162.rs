use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206162: FileFormat = FileFormat {
    id: 28_206_162,
    puid: "wikidata/28206162",
    name: "GEM Raster",
    extensions: &["img"],
    media_types: &["application/gem"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEB, 0x3C, 0x90, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
