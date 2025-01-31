use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826343: FileFormat = FileFormat {
    id: 27_826_343,
    puid: "wikidata/27826343",
    name: "Auxiliary file, AUX.XML variant",
    extensions: &["aux.xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
