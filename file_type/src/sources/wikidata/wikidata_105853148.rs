use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853148: FileFormat = FileFormat {
    id: 105_853_148,
    puid: "wikidata/105853148",
    name: "SeqBox container (v1)",
    extensions: &["sbx"],
    media_types: &["application/x-sbx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x78, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
