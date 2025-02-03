use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538998: FileFormat = FileFormat {
    id: 47_538_998,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Landscape Library",
    extensions: &["lli"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
