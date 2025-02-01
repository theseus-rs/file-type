use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48815175: FileFormat = FileFormat {
    id: 48_815_175,
    puid: "wikidata/48815175",
    name: "Framework Database, version 2",
    extensions: &["fw", "fw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
