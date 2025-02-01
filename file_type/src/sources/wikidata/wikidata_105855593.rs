use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855593: FileFormat = FileFormat {
    id: 105_855_593,
    puid: "wikidata/105855593",
    name: "Orbiter mesh",
    extensions: &["msh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x48, 0x58, 0x31, 0x0D, 0x0A, 0x47, 0x52, 0x4F, 0x55, 0x50, 0x53,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
