use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130373735: FileFormat = FileFormat {
    id: 130_373_735,
    puid: "wikidata/130373735",
    name: "Nginx configuration file",
    extensions: &["nginx.conf"],
    media_types: &["text/x-nginx-conf"],
    internal_signatures: &[],
    related_formats: &[],
};
