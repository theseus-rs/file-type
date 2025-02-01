use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50825548: FileFormat = FileFormat {
    id: 50_825_548,
    puid: "wikidata/50825548",
    name: "AVCHD Playlist File",
    extensions: &["mpl", "mpls"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
