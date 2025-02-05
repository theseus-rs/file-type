use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862273: FileFormat = FileFormat {
    id: 105_862_273,
    source_type: SourceType::Wikidata,
    name: "MegaZeux world data (v1.0x)",
    extensions: &["mzx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
