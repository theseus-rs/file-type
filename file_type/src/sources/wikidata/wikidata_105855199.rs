use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855199: FileFormat = FileFormat {
    id: 105_855_199,
    puid: "wikidata/105855199",
    name: "Windows Explorer saved search",
    extensions: &["fnd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x46, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x24,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
