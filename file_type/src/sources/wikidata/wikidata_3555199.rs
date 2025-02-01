use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3555199: FileFormat = FileFormat {
    id: 3_555_199,
    puid: "wikidata/3555199",
    name: "VEG",
    extensions: &["veg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x69, 0x66, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
