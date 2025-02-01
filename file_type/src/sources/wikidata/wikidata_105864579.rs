use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864579: FileFormat = FileFormat {
    id: 105_864_579,
    puid: "wikidata/105864579",
    name: "WSUS Patch Storage File",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4D, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
