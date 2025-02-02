use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856427: FileFormat = FileFormat {
    id: 105_856_427,
    source_type: SourceType::Wikidata,
    name: "IBM iSeries Client Access WorkStation profile",
    extensions: &["ws"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
