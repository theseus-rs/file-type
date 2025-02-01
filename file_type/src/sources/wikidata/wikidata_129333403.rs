use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129333403: FileFormat = FileFormat {
    id: 129_333_403,
    puid: "wikidata/129333403",
    name: "Gettext catalog file",
    extensions: &["pot", "pot", "pot"],
    media_types: &["application/x-gettext", "text/gettext", "text/x-gettext"],
    internal_signatures: &[],
    related_formats: &[],
};
