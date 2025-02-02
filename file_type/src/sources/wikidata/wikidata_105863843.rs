use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863843: FileFormat = FileFormat {
    id: 105_863_843,
    source_type: SourceType::Wikidata,
    name: "Multiplan for Xenix spreadsheet (v2.x)",
    extensions: &["mod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0xEC, 0x00, 0x00, 0x08, 0xAB, 0x08, 0x00, 0x1F, 0x00, 0x1A, 0x00, 0x03,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
