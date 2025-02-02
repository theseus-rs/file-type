use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850253: FileFormat = FileFormat {
    id: 105_850_253,
    source_type: SourceType::Wikidata,
    name: "AGS game configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x73, 0x6F, 0x75, 0x6E, 0x64, 0x5D, 0x0D, 0x0A, 0x64, 0x69, 0x67, 0x69,
                    0x69, 0x64, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
