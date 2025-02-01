use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129423705: FileFormat = FileFormat {
    id: 129_423_705,
    puid: "wikidata/129423705",
    name: "Gosu source code file",
    extensions: &["gs"],
    media_types: &["text/x-gosu"],
    internal_signatures: &[],
    related_formats: &[],
};
