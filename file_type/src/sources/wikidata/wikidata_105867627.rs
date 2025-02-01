use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867627: FileFormat = FileFormat {
    id: 105_867_627,
    puid: "wikidata/105867627",
    name: "NTitler show",
    extensions: &["nt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x54, 0x39, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
