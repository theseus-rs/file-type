use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50374901: FileFormat = FileFormat {
    id: 50_374_901,
    source_type: SourceType::Wikidata,
    name: "MagicaVoxel Vox format",
    extensions: &["vox"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x4F, 0x58, 0x20, 0x96, 0x00, 0x00, 0x00, 0x4D, 0x41, 0x49, 0x4E, 0x00,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
