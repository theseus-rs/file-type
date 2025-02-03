use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52063393: FileFormat = FileFormat {
    id: 52_063_393,
    source_type: SourceType::Wikidata,
    name: "TeX Binary File",
    extensions: &["dvi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
