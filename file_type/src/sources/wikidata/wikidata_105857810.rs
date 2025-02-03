use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857810: FileFormat = FileFormat {
    id: 105_857_810,
    source_type: SourceType::Wikidata,
    name: "Toast disk image (old)",
    extensions: &["toast"],
    media_types: &["application/x-roxio-toast"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x52, 0x02, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
