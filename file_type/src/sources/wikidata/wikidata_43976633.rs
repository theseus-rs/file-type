use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43976633: FileFormat = FileFormat {
    id: 43_976_633,
    puid: "wikidata/43976633",
    name: "Exchangeable Image File Format (Audio)",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
