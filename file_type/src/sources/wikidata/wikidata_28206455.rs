use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206455: FileFormat = FileFormat {
    id: 28_206_455,
    puid: "wikidata/28206455",
    name: "CKiSS",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
