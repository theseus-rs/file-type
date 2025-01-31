use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857148: FileFormat = FileFormat {
    id: 105_857_148,
    puid: "wikidata/105857148",
    name: "Microsoft HTML Help Project (minimal)",
    extensions: &["hhp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4F, 0x50, 0x54, 0x49, 0x4F, 0x4E, 0x53, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
