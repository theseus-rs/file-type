use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850874: FileFormat = FileFormat {
    id: 105_850_874,
    source_type: SourceType::Wikidata,
    name: "Keyman keyboard source",
    extensions: &["kmn"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x28, 0x26, 0x56, 0x45, 0x52,
                    0x53, 0x49, 0x4F, 0x4E, 0x29, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
