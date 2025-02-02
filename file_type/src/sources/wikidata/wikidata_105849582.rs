use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849582: FileFormat = FileFormat {
    id: 105_849_582,
    source_type: SourceType::Wikidata,
    name: "Casio calculator Tape format",
    extensions: &["cat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x52, 0x65, 0x63, 0x6F, 0x72,
                    0x64, 0x0D, 0x0A, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
