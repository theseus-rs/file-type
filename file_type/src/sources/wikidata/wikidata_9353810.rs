use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_9353810: FileFormat = FileFormat {
    id: 9_353_810,
    puid: "wikidata/9353810",
    name: "Oracle Database Trace File",
    extensions: &["trc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
