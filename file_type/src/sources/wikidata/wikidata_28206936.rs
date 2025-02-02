use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206936: FileFormat = FileFormat {
    id: 28_206_936,
    source_type: SourceType::Wikidata,
    name: "Photo CD",
    extensions: &["pcd"],
    media_types: &["image/x-photo-cd"],
    internal_signatures: &[],
    related_formats: &[],
};
