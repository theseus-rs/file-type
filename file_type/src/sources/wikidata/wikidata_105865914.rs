use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865914: FileFormat = FileFormat {
    id: 105_865_914,
    puid: "wikidata/105865914",
    name: "Palm Query Application",
    extensions: &["pqa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x71, 0x61, 0x20, 0x63, 0x6C, 0x70, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
