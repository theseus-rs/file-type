use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341451: FileFormat = FileFormat {
    id: 111_341_451,
    puid: "wikidata/111341451",
    name: "ScreamTracker v3 sample",
    extensions: &["s3i"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
