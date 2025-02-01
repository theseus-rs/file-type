use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859334: FileFormat = FileFormat {
    id: 105_859_334,
    puid: "wikidata/105859334",
    name: "QuickCompression compressed data (LE)",
    extensions: &["pmcq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4D, 0x43, 0x51, 0x01, 0x00, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
