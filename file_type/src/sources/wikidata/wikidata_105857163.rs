use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857163: FileFormat = FileFormat {
    id: 105_857_163,
    source_type: SourceType::Wikidata,
    name: "HelpScribble Project",
    extensions: &["hsc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x65, 0x6C, 0x70, 0x53, 0x63, 0x72, 0x69, 0x62, 0x62, 0x6C, 0x65, 0x20,
                    0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x2E,
                    0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
