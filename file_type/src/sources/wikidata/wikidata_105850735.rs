use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850735: FileFormat = FileFormat {
    id: 105_850_735,
    source_type: SourceType::Wikidata,
    name: "Karaoke Song List Creator song list",
    extensions: &["ksl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x22, 0x4B, 0x41, 0x52, 0x41, 0x4F, 0x4B, 0x45, 0x20, 0x53, 0x4F, 0x4E, 0x47,
                    0x20, 0x4C, 0x49, 0x53, 0x54, 0x20, 0x43, 0x52, 0x45, 0x41, 0x54, 0x4F, 0x52,
                    0x20, 0x43, 0x44, 0x20, 0x53, 0x45, 0x4C, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E,
                    0x20, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
