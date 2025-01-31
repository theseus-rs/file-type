use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61984337: FileFormat = FileFormat {
    id: 61_984_337,
    puid: "wikidata/61984337",
    name: "Microsoft Visual FoxPro database container (table files)",
    extensions: &["dbc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
