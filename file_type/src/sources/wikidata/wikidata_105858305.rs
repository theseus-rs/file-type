use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858305: FileFormat = FileFormat {
    id: 105_858_305,
    source_type: SourceType::Wikidata,
    name: "EbSynth project",
    extensions: &["ebs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x42, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
