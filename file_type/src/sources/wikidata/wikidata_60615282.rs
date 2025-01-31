use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60615282: FileFormat = FileFormat {
    id: 60_615_282,
    puid: "wikidata/60615282",
    name: "Write for Windows Document, version 3",
    extensions: &["wri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
