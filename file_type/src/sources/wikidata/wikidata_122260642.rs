use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122260642: FileFormat = FileFormat {
    id: 122_260_642,
    puid: "wikidata/122260642",
    name: "JACOsub",
    extensions: &["jss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
