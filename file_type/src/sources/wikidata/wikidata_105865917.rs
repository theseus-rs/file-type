use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865917: FileFormat = FileFormat {
    id: 105_865_917,
    puid: "wikidata/105865917",
    name: "Linux PC Screen Font data (PSF1)",
    extensions: &["psf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x36, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
