use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865047: FileFormat = FileFormat {
    id: 105_865_047,
    source_type: SourceType::Wikidata,
    name: "Paradox Update File",
    extensions: &["puf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x72, 0x61, 0x64, 0x6F, 0x78, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x3A,
                    0x20, 0x50, 0x61, 0x72, 0x61, 0x64, 0x6F, 0x78, 0x20, 0x55, 0x70, 0x64, 0x61,
                    0x74, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x50, 0x55, 0x46, 0x20,
                    0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
