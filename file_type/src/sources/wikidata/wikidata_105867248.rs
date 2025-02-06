use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867248: FileFormat = FileFormat {
    id: 105_867_248,
    source_type: SourceType::Wikidata,
    name: "Haines NFF scene (with rem)",
    extensions: &["nff"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
