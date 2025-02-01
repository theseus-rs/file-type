use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857210: FileFormat = FileFormat {
    id: 105_857_210,
    puid: "wikidata/105857210",
    name: "Alpha Four Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x34, 0x48, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
