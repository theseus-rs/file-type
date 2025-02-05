use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206936: FileFormat = FileFormat {
    id: 28_206_936,
    source_type: SourceType::Wikidata,
    name: "Photo CD",
    extensions: &["pcd"],
    media_types: &["image/x-photo-cd"],
    signatures: &[],
    related_formats: &[],
};
