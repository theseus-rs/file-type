use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105822792: FileFormat = FileFormat {
    id: 105_822_792,
    puid: "wikidata/105822792",
    name: "AMDIS Target Compounds Library",
    extensions: &["MSL"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
