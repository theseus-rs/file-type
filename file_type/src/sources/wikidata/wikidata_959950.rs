use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_959950: FileFormat = FileFormat {
    id: 959_950,
    puid: "wikidata/959950",
    name: "eXtensible Business Reporting Language",
    extensions: &["xbrl", "xml"],
    media_types: &["application/xml", "application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
