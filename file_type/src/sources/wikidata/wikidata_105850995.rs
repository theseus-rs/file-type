use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850995: FileFormat = FileFormat {
    id: 105_850_995,
    source_type: SourceType::Wikidata,
    name: "SACD 2 channel TOC",
    extensions: &["toc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x57, 0x4F, 0x43, 0x48, 0x54, 0x4F, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
