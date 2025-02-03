use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858632: FileFormat = FileFormat {
    id: 105_858_632,
    source_type: SourceType::Wikidata,
    name: "Microsoft SQL Server Backup (compressed)",
    extensions: &["bak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x53, 0x51, 0x4C, 0x42, 0x41, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
