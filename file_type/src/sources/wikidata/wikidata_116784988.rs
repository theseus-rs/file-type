use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116784988: FileFormat = FileFormat {
    id: 116_784_988,
    puid: "wikidata/116784988",
    name: "Project Manager Pro Template file",
    extensions: &["pmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
