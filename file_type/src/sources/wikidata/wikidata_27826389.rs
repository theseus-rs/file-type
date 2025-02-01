use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826389: FileFormat = FileFormat {
    id: 27_826_389,
    puid: "wikidata/27826389",
    name: "Proxy AUX file, AUX variant",
    extensions: &["aux"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
