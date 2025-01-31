use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861182: FileFormat = FileFormat {
    id: 105_861_182,
    puid: "wikidata/105861182",
    name: "Siemens ORSI Log",
    extensions: &["log"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x52, 0x53, 0x49, 0x4C, 0x4F, 0x47, 0x46, 0x49, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
