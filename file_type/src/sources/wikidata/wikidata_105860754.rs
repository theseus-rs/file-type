use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860754: FileFormat = FileFormat {
    id: 105_860_754,
    source_type: SourceType::Wikidata,
    name: "LigPlot Residue Centres-of-Mass data",
    extensions: &["rcm"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                    0x20, 0x20, 0x20, 0x20, 0x52, 0x65, 0x73, 0x2E, 0x20, 0x20, 0x52, 0x65, 0x73,
                    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x46, 0x6C, 0x61, 0x74, 0x74, 0x65,
                    0x6E, 0x65, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
