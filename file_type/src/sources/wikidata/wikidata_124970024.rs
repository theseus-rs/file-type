use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124970024: FileFormat = FileFormat {
    id: 124_970_024,
    puid: "wikidata/124970024",
    name: "MIX metadata file",
    extensions: &["mixmeta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
