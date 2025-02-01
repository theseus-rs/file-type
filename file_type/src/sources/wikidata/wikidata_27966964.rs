use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966964: FileFormat = FileFormat {
    id: 27_966_964,
    puid: "wikidata/27966964",
    name: "4X IMA ADPCM",
    extensions: &["4xm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
