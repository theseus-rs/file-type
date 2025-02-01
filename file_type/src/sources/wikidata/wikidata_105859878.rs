use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859878: FileFormat = FileFormat {
    id: 105_859_878,
    puid: "wikidata/105859878",
    name: "NETGEN Volume format",
    extensions: &["vol"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x65, 0x73, 0x68, 0x33, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
