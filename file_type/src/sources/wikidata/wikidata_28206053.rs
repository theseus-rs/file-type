use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206053: FileFormat = FileFormat {
    id: 28_206_053,
    puid: "wikidata/28206053",
    name: "ERDAS LAN",
    extensions: &["lan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
