use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125045112: FileFormat = FileFormat {
    id: 125_045_112,
    puid: "wikidata/125045112",
    name: "Yoshimi Patch Set File",
    extensions: &["xmz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
