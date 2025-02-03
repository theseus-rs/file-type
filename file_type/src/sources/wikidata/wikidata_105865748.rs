use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865748: FileFormat = FileFormat {
    id: 105_865_748,
    source_type: SourceType::Wikidata,
    name: "Ulead Imageiio/Imaginfo thumbnail",
    extensions: &["pe4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0x4F, 0x32, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
