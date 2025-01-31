use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34737296: FileFormat = FileFormat {
    id: 34_737_296,
    puid: "wikidata/34737296",
    name: "Skencil SK",
    extensions: &["sk"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x53, 0x6B, 0x65, 0x74, 0x63, 0x68, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
