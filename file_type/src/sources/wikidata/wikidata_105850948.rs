use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850948: FileFormat = FileFormat {
    id: 105_850_948,
    source_type: SourceType::Wikidata,
    name: "Track Record Viewer TRV/TRVX Index",
    extensions: &["trx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x52, 0x58, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
