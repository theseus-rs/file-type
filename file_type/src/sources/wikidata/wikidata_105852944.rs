use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852944: FileFormat = FileFormat {
    id: 105_852_944,
    source_type: SourceType::Wikidata,
    name: "SeqBox container (gen)",
    extensions: &["sbx"],
    media_types: &["application/x-sbx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
