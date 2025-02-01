use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858941: FileFormat = FileFormat {
    id: 105_858_941,
    puid: "wikidata/105858941",
    name: "ST-6 compressed astrocamera bitmap",
    extensions: &["st6"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x2D, 0x36, 0x20, 0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73,
                    0x65, 0x64, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
