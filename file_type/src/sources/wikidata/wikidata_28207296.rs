use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207296: FileFormat = FileFormat {
    id: 28_207_296,
    puid: "wikidata/28207296",
    name: "Spectrum 512 Extended",
    extensions: &["spx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
