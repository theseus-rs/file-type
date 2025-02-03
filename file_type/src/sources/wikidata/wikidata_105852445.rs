use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852445: FileFormat = FileFormat {
    id: 105_852_445,
    source_type: SourceType::Wikidata,
    name: "Sxz hybrid vector/raster image",
    extensions: &["sxz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x58, 0x5A, 0x46, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
