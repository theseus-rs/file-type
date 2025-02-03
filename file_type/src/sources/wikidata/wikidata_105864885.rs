use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864885: FileFormat = FileFormat {
    id: 105_864_885,
    source_type: SourceType::Wikidata,
    name: "Programmer's Notepad Project Group",
    extensions: &["ppg"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x6E, 0x61,
                    0x6D, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
