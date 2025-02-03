use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867083: FileFormat = FileFormat {
    id: 105_867_083,
    source_type: SourceType::Wikidata,
    name: "FreePCB Netlist",
    extensions: &["net"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x50, 0x41, 0x44, 0x53, 0x2D, 0x50, 0x43, 0x42, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
