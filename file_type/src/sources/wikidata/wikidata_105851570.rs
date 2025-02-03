use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851570: FileFormat = FileFormat {
    id: 105_851_570,
    source_type: SourceType::Wikidata,
    name: "Speedo font Typeface Definition File",
    extensions: &["tdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x65, 0x6E, 0x75, 0x6C, 0x61, 0x62, 0x65, 0x6C, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
