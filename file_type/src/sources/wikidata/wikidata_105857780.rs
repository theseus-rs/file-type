use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857780: FileFormat = FileFormat {
    id: 105_857_780,
    source_type: SourceType::Wikidata,
    name: "KLH10 RAW tape image directory",
    extensions: &["tdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x46, 0x2D, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3A, 0x20, 0x72, 0x61,
                    0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
