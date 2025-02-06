use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063393: FileFormat = FileFormat {
    id: 52_063_393,
    source_type: SourceType::Wikidata,
    name: "TeX Binary File",
    extensions: &["dvi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
