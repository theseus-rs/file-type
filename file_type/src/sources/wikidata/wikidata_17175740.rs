use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17175740: FileFormat = FileFormat {
    id: 17_175_740,
    puid: "wikidata/17175740",
    name: "comic book archive, tar container",
    extensions: &["cbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
