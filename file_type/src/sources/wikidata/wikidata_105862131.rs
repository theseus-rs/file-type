use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862131: FileFormat = FileFormat {
    id: 105_862_131,
    puid: "wikidata/105862131",
    name: "Emacs Muse project",
    extensions: &["muse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
