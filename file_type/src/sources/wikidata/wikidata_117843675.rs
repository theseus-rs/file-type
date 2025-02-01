use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117843675: FileFormat = FileFormat {
    id: 117_843_675,
    puid: "wikidata/117843675",
    name: "Wicat file",
    extensions: &["ged"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
