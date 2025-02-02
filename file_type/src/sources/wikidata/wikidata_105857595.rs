use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857595: FileFormat = FileFormat {
    id: 105_857_595,
    source_type: SourceType::Wikidata,
    name: "CopyControl I.C.A. disk image",
    extensions: &["ica"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x43, 0x4F, 0x4E, 0x54, 0x52, 0x4F, 0x4C, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
