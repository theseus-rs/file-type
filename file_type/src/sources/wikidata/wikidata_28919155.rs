use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919155: FileFormat = FileFormat {
    id: 28_919_155,
    puid: "wikidata/28919155",
    name: "Rhino Worksession",
    extensions: &["rws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
