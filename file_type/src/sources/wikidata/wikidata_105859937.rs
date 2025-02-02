use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859937: FileFormat = FileFormat {
    id: 105_859_937,
    source_type: SourceType::Wikidata,
    name: "WAP Bookmark info",
    extensions: &["vbm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x42, 0x4B, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
