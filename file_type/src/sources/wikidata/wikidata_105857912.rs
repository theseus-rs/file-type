use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857912: FileFormat = FileFormat {
    id: 105_857_912,
    puid: "wikidata/105857912",
    name: "Amigaguide Index",
    extensions: &["index"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x79, 0x70, 0x65, 0x72, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
