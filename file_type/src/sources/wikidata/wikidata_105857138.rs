use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857138: FileFormat = FileFormat {
    id: 105_857_138,
    puid: "wikidata/105857138",
    name: "HDF IDE hard disk image",
    extensions: &["hdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x2D, 0x49, 0x44, 0x45, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
