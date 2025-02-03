use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860511: FileFormat = FileFormat {
    id: 105_860_511,
    source_type: SourceType::Wikidata,
    name: "IDRISI raster image Reference",
    extensions: &["ref"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x72, 0x65, 0x66, 0x2E, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x20, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
