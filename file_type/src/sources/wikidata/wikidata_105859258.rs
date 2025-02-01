use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859258: FileFormat = FileFormat {
    id: 105_859_258,
    puid: "wikidata/105859258",
    name: "Biovision Action data",
    extensions: &["bva"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x65, 0x67, 0x6D, 0x65, 0x6E, 0x74, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
