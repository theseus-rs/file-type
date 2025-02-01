use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864882: FileFormat = FileFormat {
    id: 105_864_882,
    puid: "wikidata/105864882",
    name: "Virtual T IDE Project",
    extensions: &["prj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x41, 0x4D, 0x45, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
