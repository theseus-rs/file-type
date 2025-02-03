use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865413: FileFormat = FileFormat {
    id: 105_865_413,
    source_type: SourceType::Wikidata,
    name: "AmigaKonto Preferences",
    extensions: &["prefs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x47, 0x41, 0x4B, 0x4F, 0x4E, 0x54, 0x4F, 0x5F, 0x50, 0x52,
                    0x45, 0x46, 0x53, 0x5F, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
