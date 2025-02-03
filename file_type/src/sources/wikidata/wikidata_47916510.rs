use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47916510: FileFormat = FileFormat {
    id: 47_916_510,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Web Query",
    extensions: &["iqy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x45, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
