use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866241: FileFormat = FileFormat {
    id: 105_866_241,
    source_type: SourceType::Wikidata,
    name: "Detached PDS Label info (v3)",
    extensions: &["lbl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x44, 0x53, 0x5F, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x5F, 0x49,
                    0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
