use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852879: FileFormat = FileFormat {
    id: 105_852_879,
    source_type: SourceType::Wikidata,
    name: "Term95 Script",
    extensions: &["scx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
