use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850429: FileFormat = FileFormat {
    id: 105_850_429,
    puid: "wikidata/105850429",
    name: "Cricket Audio XML Bank Description",
    extensions: &["ckbx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
