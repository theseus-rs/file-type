use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66146236: FileFormat = FileFormat {
    id: 66_146_236,
    puid: "wikidata/66146236",
    name: "InfoPath Form Definition File",
    extensions: &["xsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
