use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853375: FileFormat = FileFormat {
    id: 105_853_375,
    puid: "wikidata/105853375",
    name: "Swift Interchange File V2",
    extensions: &["sif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x53, 0x57, 0x49, 0x46, 0x54, 0x20, 0x49, 0x4E, 0x54, 0x45, 0x52,
                    0x43, 0x48, 0x41, 0x4E, 0x47, 0x45, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x56,
                    0x32, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
