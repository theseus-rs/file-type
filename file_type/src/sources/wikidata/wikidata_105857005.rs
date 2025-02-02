use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857005: FileFormat = FileFormat {
    id: 105_857_005,
    source_type: SourceType::Wikidata,
    name: "SBG Object Text",
    extensions: &["geo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x6C, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
