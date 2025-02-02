use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975915: FileFormat = FileFormat {
    id: 28_975_915,
    source_type: SourceType::Wikidata,
    name: "ZPR",
    extensions: &["zpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
