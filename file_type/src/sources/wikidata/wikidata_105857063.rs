use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857063: FileFormat = FileFormat {
    id: 105_857_063,
    puid: "wikidata/105857063",
    name: "GIMP Gradient",
    extensions: &["ggr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x49, 0x4D, 0x50, 0x20, 0x47, 0x72, 0x61, 0x64, 0x69, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
