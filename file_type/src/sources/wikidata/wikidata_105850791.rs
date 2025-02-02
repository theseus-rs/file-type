use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850791: FileFormat = FileFormat {
    id: 105_850_791,
    source_type: SourceType::Wikidata,
    name: "KDevelop Project (with rem)",
    extensions: &["kdevprj"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x4B, 0x44, 0x45, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x0A, 0x5B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
