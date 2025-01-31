use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867194: FileFormat = FileFormat {
    id: 105_867_194,
    puid: "wikidata/105867194",
    name: "NovoTrade Packer module",
    extensions: &["ntp"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x44, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
