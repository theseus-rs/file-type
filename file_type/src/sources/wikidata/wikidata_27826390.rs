use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826390: FileFormat = FileFormat {
    id: 27_826_390,
    puid: "wikidata/27826390",
    name: "Proxy AUX file, AUX.XML variant",
    extensions: &["aux.xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
