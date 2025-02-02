use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860994: FileFormat = FileFormat {
    id: 105_860_994,
    source_type: SourceType::Wikidata,
    name: "Footprint/IBM Works Data Filer DataBase",
    extensions: &["ldb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x8B, 0x5D, 0x09])],
            },
        }],
    }],
    related_formats: &[],
};
