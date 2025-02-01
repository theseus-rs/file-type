use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124970136: FileFormat = FileFormat {
    id: 124_970_136,
    puid: "wikidata/124970136",
    name: "MIX status file",
    extensions: &["mixstatus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
