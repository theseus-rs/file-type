use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857242: FileFormat = FileFormat {
    id: 105_857_242,
    puid: "wikidata/105857242",
    name: "Altera Hierarchy Interconnect File",
    extensions: &["hif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x49, 0x46, 0x30, 0x30, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
