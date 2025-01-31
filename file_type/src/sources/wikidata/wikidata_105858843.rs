use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858843: FileFormat = FileFormat {
    id: 105_858_843,
    puid: "wikidata/105858843",
    name: "KiCad pcbnew PCB layout",
    extensions: &["brd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x42, 0x4E, 0x45, 0x57, 0x2D, 0x42, 0x4F, 0x41, 0x52, 0x44, 0x20,
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
