use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863567: FileFormat = FileFormat {
    id: 105_863_567,
    puid: "wikidata/105863567",
    name: "SunVox Synthesizer",
    extensions: &["sunsynth"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x53, 0x59, 0x4E, 0x00, 0x00, 0x00, 0x00, 0x56, 0x45, 0x52, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
