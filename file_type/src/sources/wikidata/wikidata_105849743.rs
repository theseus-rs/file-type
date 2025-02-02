use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849743: FileFormat = FileFormat {
    id: 105_849_743,
    source_type: SourceType::Wikidata,
    name: "EzCad Configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x45, 0x5A, 0x43, 0x41, 0x44, 0x5F, 0x43, 0x46, 0x47, 0x5D, 0x0D, 0x0A,
                    0x50, 0x52, 0x49, 0x4D, 0x41, 0x52, 0x59, 0x4C, 0x41, 0x4E, 0x47, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
