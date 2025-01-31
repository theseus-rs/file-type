use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853422: FileFormat = FileFormat {
    id: 105_853_422,
    puid: "wikidata/105853422",
    name: "SETI@Home Classic work unit",
    extensions: &["sah"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x74, 0x79, 0x70, 0x65, 0x3D, 0x77, 0x6F, 0x72, 0x6B, 0x20, 0x75, 0x6E, 0x69,
                    0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
