use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866975: FileFormat = FileFormat {
    id: 105_866_975,
    puid: "wikidata/105866975",
    name: "Hamamatsu NanoZoomer Digital Pathology Image",
    extensions: &["ndpi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
