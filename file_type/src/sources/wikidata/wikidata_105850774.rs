use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850774: FileFormat = FileFormat {
    id: 105_850_774,
    puid: "wikidata/105850774",
    name: "Klystrack Instrument",
    extensions: &["ki"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x79, 0x64, 0x21, 0x69, 0x6E, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
