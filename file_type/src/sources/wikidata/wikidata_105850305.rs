use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850305: FileFormat = FileFormat {
    id: 105_850_305,
    puid: "wikidata/105850305",
    name: "SeeYou Raster Map",
    extensions: &["cmr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x55, 0x4D, 0x41, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
