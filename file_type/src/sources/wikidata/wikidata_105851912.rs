use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851912: FileFormat = FileFormat {
    id: 105_851_912,
    source_type: SourceType::Wikidata,
    name: "Sinclair TR-DOS disk image",
    extensions: &["scl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x4E, 0x43, 0x4C, 0x41, 0x49, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
