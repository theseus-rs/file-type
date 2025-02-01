use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853261: FileFormat = FileFormat {
    id: 105_853_261,
    puid: "wikidata/105853261",
    name: "SCC Blaffer NT SCC Instrument",
    extensions: &["sbs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x6C, 0x61, 0x66, 0x20, 0x4E, 0x54, 0x20, 0x53, 0x43, 0x43, 0x20, 0x49,
                    0x6E, 0x73, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
