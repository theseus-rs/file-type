use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473293: FileFormat = FileFormat {
    id: 27_473_293,
    puid: "wikidata/27473293",
    name: "CADRG Overview Image",
    extensions: &["ovr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
