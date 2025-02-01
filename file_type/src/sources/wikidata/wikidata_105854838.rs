use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854838: FileFormat = FileFormat {
    id: 105_854_838,
    puid: "wikidata/105854838",
    name: "Ay Emul play List",
    extensions: &["ayl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x20, 0x53, 0x70, 0x65, 0x63, 0x74, 0x72, 0x75, 0x6D, 0x20, 0x53,
                    0x6F, 0x75, 0x6E, 0x64, 0x20, 0x43, 0x68, 0x69, 0x70, 0x20, 0x45, 0x6D, 0x75,
                    0x6C, 0x61, 0x74, 0x6F, 0x72, 0x20, 0x50, 0x6C, 0x61, 0x79, 0x20, 0x4C, 0x69,
                    0x73, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
