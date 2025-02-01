use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857252: FileFormat = FileFormat {
    id: 105_857_252,
    puid: "wikidata/105857252",
    name: "Faery Tale Adventure 2 Resources",
    extensions: &["hrs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x52, 0x45, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
