use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858392: FileFormat = FileFormat {
    id: 105_858_392,
    puid: "wikidata/105858392",
    name: "eGatherer Collected System Configuration Information",
    extensions: &["eg2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x45, 0x47, 0x41, 0x54, 0x48, 0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
