use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857229: FileFormat = FileFormat {
    id: 105_857_229,
    source_type: SourceType::Wikidata,
    name: "FL Studio Hardcore Program",
    extensions: &["hdprg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x52, 0x43, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
