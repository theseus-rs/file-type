use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50825548: FileFormat = FileFormat {
    id: 50_825_548,
    source_type: SourceType::Wikidata,
    name: "AVCHD Playlist File",
    extensions: &["mpl", "mpls"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
