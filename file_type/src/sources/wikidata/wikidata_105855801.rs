use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855801: FileFormat = FileFormat {
    id: 105_855_801,
    puid: "wikidata/105855801",
    name: "AAE RomList format",
    extensions: &["dat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x41, 0x45, 0x20, 0x41, 0x6C, 0x6C, 0x20, 0x47, 0x61, 0x6D, 0x65, 0x73,
                    0x20, 0x52, 0x6F, 0x6D, 0x4C, 0x69, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
