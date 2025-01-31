use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_70081372: FileFormat = FileFormat {
    id: 70_081_372,
    puid: "wikidata/70081372",
    name: "FLAC Fingerprint file format",
    extensions: &["ffp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x66, 0x6C, 0x61, 0x63, 0x20, 0x66, 0x69, 0x6E, 0x67, 0x65, 0x72,
                    0x70, 0x72, 0x69, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
