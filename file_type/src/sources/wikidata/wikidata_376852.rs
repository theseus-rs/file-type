use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_376852: FileFormat = FileFormat {
    id: 376_852,
    puid: "wikidata/376852",
    name: "Extended Module",
    extensions: &["xm"],
    media_types: &["audio/xm"],
    internal_signatures: &[],
    related_formats: &[],
};
