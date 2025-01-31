use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859208: FileFormat = FileFormat {
    id: 105_859_208,
    puid: "wikidata/105859208",
    name: "NCSA Telnet Interactive Color Raster bitmap",
    extensions: &["icr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1B, 0x5E, 0x57, 0x3B, 0x30, 0x3B, 0x30, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
