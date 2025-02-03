use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861889: FileFormat = FileFormat {
    id: 105_861_889,
    source_type: SourceType::Wikidata,
    name: "Media Descriptor",
    extensions: &["mds"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x44, 0x49, 0x41, 0x20, 0x44, 0x45, 0x53, 0x43, 0x52, 0x49, 0x50,
                    0x54, 0x4F, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
