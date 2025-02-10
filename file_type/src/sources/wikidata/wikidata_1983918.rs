use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1983918: FileFormat = FileFormat {
    id: 1_983_918,
    source_type: SourceType::Wikidata,
    name: "Nexus file",
    extensions: &["nex", "nxs", "nxz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x73, 0x78, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
