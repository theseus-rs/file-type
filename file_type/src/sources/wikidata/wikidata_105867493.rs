use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867493: FileFormat = FileFormat {
    id: 105_867_493,
    source_type: SourceType::Wikidata,
    name: "NoiseTrekker v2.0 module",
    extensions: &["ntk"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x57, 0x4E, 0x4E, 0x53, 0x4E, 0x47, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
