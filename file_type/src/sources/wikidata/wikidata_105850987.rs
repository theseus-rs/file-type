use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850987: FileFormat = FileFormat {
    id: 105_850_987,
    puid: "wikidata/105850987",
    name: "Typed Voxel format",
    extensions: &["tox"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x6F, 0x78, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
