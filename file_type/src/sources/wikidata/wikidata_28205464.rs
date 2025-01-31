use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205464: FileFormat = FileFormat {
    id: 28_205_464,
    puid: "wikidata/28205464",
    name: "PSF",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x50, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
