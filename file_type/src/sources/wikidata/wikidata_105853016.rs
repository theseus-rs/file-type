use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853016: FileFormat = FileFormat {
    id: 105_853_016,
    puid: "wikidata/105853016",
    name: "SkyOS Installation File",
    extensions: &["sif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x47, 0x45, 0x4E, 0x45, 0x52, 0x41, 0x4C, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
