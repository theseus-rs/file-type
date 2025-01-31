use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865835: FileFormat = FileFormat {
    id: 105_865_835,
    puid: "wikidata/105865835",
    name: "PSP metadata file",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x50, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
