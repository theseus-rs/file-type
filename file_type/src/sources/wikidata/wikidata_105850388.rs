use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850388: FileFormat = FileFormat {
    id: 105_850_388,
    source_type: SourceType::Wikidata,
    name: "Celestia 3D model (binary)",
    extensions: &["cmod"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x63, 0x65, 0x6C, 0x6D, 0x6F, 0x64, 0x65, 0x6C, 0x5F, 0x62, 0x69, 0x6E,
                    0x61, 0x72, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
