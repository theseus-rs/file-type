use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855541: FileFormat = FileFormat {
    id: 105_855_541,
    puid: "wikidata/105855541",
    name: "Blender 3D object",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x42, 0x6C, 0x65, 0x6E, 0x64, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
