use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850739: FileFormat = FileFormat {
    id: 105_850_739,
    puid: "wikidata/105850739",
    name: "Kodak Digital Camera RAW image (EasyShare series)",
    extensions: &["kdc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00, 0x14,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
