use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33073902: FileFormat = FileFormat {
    id: 33_073_902,
    puid: "wikidata/33073902",
    name: "Offline Web applications",
    extensions: &["appcache"],
    media_types: &["text/cache-manifest"],
    internal_signatures: &[],
    related_formats: &[],
};
