use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849884: FileFormat = FileFormat {
    id: 105_849_884,
    puid: "wikidata/105849884",
    name: "SRI PeakSimple chromatogram",
    extensions: &["chr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x54, 0x59, 0x50, 0x45, 0x3E, 0x3D, 0x43, 0x48, 0x52, 0x4F, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
