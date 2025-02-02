use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857929: FileFormat = FileFormat {
    id: 105_857_929,
    source_type: SourceType::Wikidata,
    name: "AmiAtlas Index",
    extensions: &["index"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x41, 0x54, 0x4C, 0x41, 0x53, 0x5F, 0x49,
                    0x4E, 0x44, 0x45, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
