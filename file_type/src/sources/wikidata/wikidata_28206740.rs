use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206740: FileFormat = FileFormat {
    id: 28_206_740,
    puid: "wikidata/28206740",
    name: "Navy Image File Format",
    extensions: &["ct3", "nif"],
    media_types: &["image/x-niff", "image/x-niff"],
    internal_signatures: &[],
    related_formats: &[],
};
