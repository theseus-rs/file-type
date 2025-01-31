use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85708317: FileFormat = FileFormat {
    id: 85_708_317,
    puid: "wikidata/85708317",
    name: "Calendar Creator File",
    extensions: &["cc3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
