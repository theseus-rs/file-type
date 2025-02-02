use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863461: FileFormat = FileFormat {
    id: 105_863_461,
    source_type: SourceType::Wikidata,
    name: "MS Developer Studio Project",
    extensions: &["dsp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x44,
                    0x65, 0x76, 0x65, 0x6C, 0x6F, 0x70, 0x65, 0x72, 0x20, 0x53, 0x74, 0x75, 0x64,
                    0x69, 0x6F, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x46, 0x69,
                    0x6C, 0x65, 0x20, 0x2D, 0x20, 0x4E, 0x61, 0x6D, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
