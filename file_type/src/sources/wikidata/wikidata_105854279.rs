use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854279: FileFormat = FileFormat {
    id: 105_854_279,
    puid: "wikidata/105854279",
    name: "A.M.Composer 1.2 music",
    extensions: &["amc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x43, 0x20, 0x56, 0x31, 0x2E, 0x32, 0x20, 0x52, 0x45, 0x50, 0x4C,
                    0x41, 0x59, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
