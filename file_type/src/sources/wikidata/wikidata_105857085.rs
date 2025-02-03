use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857085: FileFormat = FileFormat {
    id: 105_857_085,
    source_type: SourceType::Wikidata,
    name: "Granite Devices Firmware (v310)",
    extensions: &["gdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x44, 0x46, 0x57, 0x36, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
