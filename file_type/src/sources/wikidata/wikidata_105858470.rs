use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858470: FileFormat = FileFormat {
    id: 105_858_470,
    puid: "wikidata/105858470",
    name: "H.R.C.D.T. warrior load format",
    extensions: &["exe"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x2E, 0x52, 0x2E, 0x43, 0x2E, 0x44, 0x2E, 0x54, 0x2E, 0x2D, 0x53, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
