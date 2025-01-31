use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975633: FileFormat = FileFormat {
    id: 28_975_633,
    puid: "wikidata/28975633",
    name: "NextEngine Scan",
    extensions: &["scn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
