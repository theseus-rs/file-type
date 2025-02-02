use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975654: FileFormat = FileFormat {
    id: 28_975_654,
    source_type: SourceType::Wikidata,
    name: "Recon Points",
    extensions: &["pts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
