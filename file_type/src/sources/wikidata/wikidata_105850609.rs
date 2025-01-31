use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850609: FileFormat = FileFormat {
    id: 105_850_609,
    puid: "wikidata/105850609",
    name: "Lotus Symphony configuration",
    extensions: &["cnf"],
    media_types: &["application/vnd.lotus-1-2-3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x02, 0x08])],
            },
        }],
    }],
    related_formats: &[],
};
