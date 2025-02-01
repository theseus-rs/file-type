use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206795: FileFormat = FileFormat {
    id: 28_206_795,
    puid: "wikidata/28206795",
    name: "OS/2 Boot Logo",
    extensions: &["lgo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
