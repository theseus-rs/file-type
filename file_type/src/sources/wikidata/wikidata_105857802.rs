use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857802: FileFormat = FileFormat {
    id: 105_857_802,
    source_type: SourceType::Wikidata,
    name: "Lynx Disk Format",
    extensions: &["ldf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x79, 0x6E, 0x78, 0x20, 0x64, 0x69, 0x73, 0x6B, 0x73, 0x2C, 0x20, 0x43,
                    0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x49, 0x6E, 0x74, 0x65,
                    0x6C, 0x6C, 0x69, 0x67, 0x65, 0x6E, 0x74, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77,
                    0x61, 0x72, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
