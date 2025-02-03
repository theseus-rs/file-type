use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979392: FileFormat = FileFormat {
    id: 27_979_392,
    source_type: SourceType::Wikidata,
    name: "BAM",
    extensions: &["bam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x4D, 0x20, 0x56, 0x32, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
