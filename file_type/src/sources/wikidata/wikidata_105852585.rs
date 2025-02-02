use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852585: FileFormat = FileFormat {
    id: 105_852_585,
    source_type: SourceType::Wikidata,
    name: "League Of Legends Skeleton",
    extensions: &["skl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x72, 0x33, 0x64, 0x32, 0x73, 0x6B, 0x6C, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
