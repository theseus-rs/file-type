use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130373735: FileFormat = FileFormat {
    id: 130_373_735,
    source_type: SourceType::Wikidata,
    name: "Nginx configuration file",
    extensions: &["nginx.conf"],
    media_types: &["text/x-nginx-conf"],
    signatures: &[],
    related_formats: &[],
};
