use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864023: FileFormat = FileFormat {
    id: 105_864_023,
    puid: "wikidata/105864023",
    name: "MidiCo Karaoke",
    extensions: &["mdc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x44, 0x43, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
