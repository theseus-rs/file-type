use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853101: FileFormat = FileFormat {
    id: 105_853_101,
    source_type: SourceType::Wikidata,
    name: "Adobe Setup Installation File",
    extensions: &["sif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x64, 0x6F, 0x62, 0x65, 0x20, 0x41, 0x53, 0x4E, 0x20, 0x53, 0x49, 0x46,
                    0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
