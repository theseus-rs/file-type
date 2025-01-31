use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858273: FileFormat = FileFormat {
    id: 105_858_273,
    puid: "wikidata/105858273",
    name: ".NET Micro Framework PE executable",
    extensions: &["exe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x53, 0x70, 0x6F, 0x74, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
