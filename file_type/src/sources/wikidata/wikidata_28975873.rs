use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975873: FileFormat = FileFormat {
    id: 28_975_873,
    puid: "wikidata/28975873",
    name: "OOGL LIST file",
    extensions: &["list"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
