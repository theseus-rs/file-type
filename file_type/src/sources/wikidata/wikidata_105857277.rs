use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857277: FileFormat = FileFormat {
    id: 105_857_277,
    puid: "wikidata/105857277",
    name: "gfxboot compiled html help",
    extensions: &["hlp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0x12])],
            },
        }],
    }],
    related_formats: &[],
};
