use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205541: FileFormat = FileFormat {
    id: 28_205_541,
    puid: "wikidata/28205541",
    name: "NeoDesk icon",
    extensions: &["nic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
