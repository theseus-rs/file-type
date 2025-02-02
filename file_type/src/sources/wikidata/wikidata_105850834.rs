use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850834: FileFormat = FileFormat {
    id: 105_850_834,
    source_type: SourceType::Wikidata,
    name: "Gamebyro KFM data",
    extensions: &["kfm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x47, 0x61, 0x6D, 0x65, 0x62, 0x72, 0x79, 0x6F, 0x20, 0x4B, 0x46, 0x4D,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
