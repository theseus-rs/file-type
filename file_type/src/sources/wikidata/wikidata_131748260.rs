use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131748260: FileFormat = FileFormat {
    id: 131_748_260,
    puid: "wikidata/131748260",
    name: "Parallel Input Output file",
    extensions: &["pio"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
