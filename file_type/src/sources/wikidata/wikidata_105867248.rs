use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867248: FileFormat = FileFormat {
    id: 105_867_248,
    source_type: SourceType::Wikidata,
    name: "Haines NFF scene (with rem)",
    extensions: &["nff"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
