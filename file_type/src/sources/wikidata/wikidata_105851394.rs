use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851394: FileFormat = FileFormat {
    id: 105_851_394,
    source_type: SourceType::Wikidata,
    name: "TeamViewer Configuration",
    extensions: &["tvc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x54, 0x65, 0x61, 0x6D, 0x56, 0x69, 0x65, 0x77, 0x65, 0x72, 0x20, 0x43,
                    0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x5D,
                    0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
