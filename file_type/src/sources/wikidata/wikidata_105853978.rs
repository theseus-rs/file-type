use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853978: FileFormat = FileFormat {
    id: 105_853_978,
    puid: "wikidata/105853978",
    name: "UltraCompressor 2 Encrypted archive",
    extensions: &["ue2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x45, 0x32, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
