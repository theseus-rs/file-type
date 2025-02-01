use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959804: FileFormat = FileFormat {
    id: 27_959_804,
    puid: "wikidata/27959804",
    name: "Ableton Live Clip",
    extensions: &["alc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
