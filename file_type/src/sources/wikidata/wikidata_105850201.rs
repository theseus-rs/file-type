use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850201: FileFormat = FileFormat {
    id: 105_850_201,
    puid: "wikidata/105850201",
    name: "Comic Life Document",
    extensions: &["comiclife"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x4C, 0x6D, 0x43, 0x30, 0x30, 0x31, 0x31, 0x50, 0x61, 0x78, 0x48, 0x65,
                    0x61, 0x64, 0x65, 0x72, 0x2F, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x73,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
