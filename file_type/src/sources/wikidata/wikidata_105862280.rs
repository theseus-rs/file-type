use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862280: FileFormat = FileFormat {
    id: 105_862_280,
    source_type: SourceType::Wikidata,
    name: "Doom 3 MD5 Mesh",
    extensions: &["md5mesh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x44, 0x35, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
