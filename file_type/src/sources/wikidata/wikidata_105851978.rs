use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851978: FileFormat = FileFormat {
    id: 105_851_978,
    source_type: SourceType::Wikidata,
    name: "Hadoop SequenceFile format",
    extensions: &["seq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x45, 0x51, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
