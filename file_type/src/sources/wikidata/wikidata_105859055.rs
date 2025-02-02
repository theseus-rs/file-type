use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859055: FileFormat = FileFormat {
    id: 105_859_055,
    source_type: SourceType::Wikidata,
    name: "BannerMania banner",
    extensions: &["bnr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD1, 0xBA, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
