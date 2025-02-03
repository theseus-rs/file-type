use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4812839: FileFormat = FileFormat {
    id: 4_812_839,
    source_type: SourceType::Wikidata,
    name: "Atari SAP music format",
    extensions: &["sap"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x41, 0x50, 0x0D, 0x0A, 0x41, 0x55, 0x54, 0x48, 0x4F, 0x52, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
